#[deny(unsafe_op_in_unsafe_fn)]

use core::{
  cell::Cell,
  clone::Clone,
  fmt,
  marker::{Copy, PhantomPinned},
  ops::Deref,
  pin::Pin,
  ptr::NonNull,
};

#[repr(transparent)]
struct Readonly<T: ?Sized> {
  value: T,
}

impl<T> Readonly<T> {
  #[inline]
  pub const fn new(value: T) -> Self {
    Self {
      value,
    }
  }
  #[inline]
  pub const fn into_inner(self) -> T {
    self.value
  }
}

impl<T: ?Sized> Readonly<T> {
  #[inline]
  pub const fn as_ref(&self) -> &T {
    &self.value
  }
  #[inline]
  pub const fn as_ptr(&self) -> *const T {
    &self.value as *const T
  }
}

impl<T> From<T> for Readonly<T> {
  #[inline]
  fn from(x: T) -> Self {
    Self::new(x)
  }
}

impl<T: Default> Default for Readonly<T> {
  #[inline]
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl<T: ?Sized + core::fmt::Display> fmt::Display for Readonly<T> {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.value.fmt(f)
  }
}

impl<T: ?Sized> Deref for Readonly<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: ?Sized + Clone> Clone for Readonly<T> {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      value: self.value.clone(),
    }
  }
}

impl<T: ?Sized + Copy> Copy for Readonly<T> {}

#[repr(transparent)]
struct LifetimeErased<T: ?Sized> {
  pointer: NonNull<T>,
}

impl<T: ?Sized> Deref for LifetimeErased<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe { self.pointer.as_ref() }
  }
}

impl<T> LifetimeErased<T> {
  /// SAFETY: The underlying object must not be dropped during the returned value's lifetime, not just the pin's lifetime.
  pub const unsafe fn new_unchecked(x: Pin<&Readonly<T>>) -> Pin<Self> {
    // SAFETY: Safe because we will not touch the actual object, only pointers and references to it, which implies that
    // the pinning invariants will be not be violated.
    let x = unsafe { Pin::into_inner_unchecked(x) }.as_ptr();
    // SAFETY: Safe because x is guaranteed not to be null.
    let x = unsafe { NonNull::new_unchecked(x as *mut T) };
    let x = Self { pointer: x };
    // SAFETY: Safe because x points to a pinned object, and because the contract of this function requires that that object
    // live as long as this one will.
    unsafe { Pin::new_unchecked(x) }
  }
}

impl<T: ?Sized> LifetimeErased<T> {
  // SAFETY: The object underlying `self` must not be dropped during the returned value's lifetime.
  pub const unsafe fn clone_unchecked(&self) -> Self {
    Self {
      pointer: self.pointer.clone()
    }
  }
  pub const fn as_ptr(self) -> *mut T {
    self.pointer.as_ptr()
  }
  pub fn as_ref(&self) -> &T {
    self.pointer.as_ref()
  }
  pub fn cast<U>(self: Pin<&Self>) -> Pin<LifetimeErased<U>> {
    // Safe because all memory in 
    unsafe {
      Pin::new_unchecked(LifetimeErased::<U> {
        pointer: self.pointer.cast(),
      })
    }
  }
}

enum NodePrev<T: 'static> {
  Head(Pin<LifetimeErased<List<T>>>),
  Node(Pin<LifetimeErased<Node<T>>>),
}

impl<T> NodePrev<T> {
  fn maybe_head(&self) -> Option<Pin<LifetimeErased<List<T>>>> {
    match self {
      NodePrev::Head(x) => Some(*x),
      _ => None,
    }
  }
  fn maybe_node(&self) -> Option<Pin<LifetimeErased<Node<T>>>> {
    match self {
      NodePrev::Node(x) => Some(*x),
      _ => None,
    }
  }
}

impl<T> Clone for NodePrev<T> {
  fn clone(&self) -> Self {
    match self {
      Self::Head(head) => Self::Head(*head),
      Self::Node(node) => Self::Node(*node),
    }
  }
}
impl<T> Copy for NodePrev<T> {}

enum NodeNext<T: 'static> {
  Tail,
  Node(Pin<LifetimeErased<Node<T>>>),
}

impl<T> NodeNext<T> {
  fn maybe_tail(&self) -> Option<()> {
    match self {
      NodeNext::Tail => Some(()),
      _ => None,
    }
  }
  fn maybe_node(&self) -> Option<Pin<LifetimeErased<Node<T>>>> {
    match self {
      NodeNext::Node(x) => Some(*x),
      _ => None,
    }
  }
}

impl<T> Clone for NodeNext<T> {
  fn clone(&self) -> Self {
    match self {
      Self::Tail => Self::Tail,
      Self::Node(node) => Self::Node(*node),
    }
  }
}
impl<T> Copy for NodeNext<T> {}

struct Link<T: 'static> {
  prev: Cell<NodePrev<T>>,
  next: Cell<NodeNext<T>>,
}

pub struct Node<T: 'static> {
  value: T,
  pos: Option<Link<T>>,
  _0: PhantomPinned,
}

impl<T: 'static> Node<T> {
  pub fn new(value: T) -> Self {
    Self {
      value,
      pos: None,
      _0: PhantomPinned,
    }
  }
}

impl<T> Drop for Node<T> {
  fn drop(&mut self) {
    match self.pos {
      None => (),
      Some(ref pos) => match pos.prev.get() {
        NodePrev::Head(head) => {
          match pos.next.get() {
            NodeNext::Tail => {
              // Current node is the first and only one in the chain
              let old_head_head = head.head.replace(None);
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_head_head.unwrap()).as_ptr()
                );
              }
            }
            NodeNext::Node(next) => {
              // Current node is the first node, but there's another after it
              let old_next_prev = next.pos.as_ref().unwrap().prev.replace(NodePrev::Head(head));
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_next_prev.maybe_node().unwrap()).as_ptr()
                    as *const Node<T>
                );
              }
              let old_head_head = head.head.replace(Some(next));
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_head_head.unwrap()).as_ptr() as *const Node<T>
                );
              }
            }
          }
        }
        NodePrev::Node(prev) => {
          match pos.next.get() {
            NodeNext::Tail => {
              // Current node is not the first one, but is the last
              let old_prev_next = prev.pos.as_ref().unwrap().next.replace(NodeNext::Tail);
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_prev_next.maybe_node().unwrap()).as_ptr()
                    as *const Node<T>
                );
              }
            }
            NodeNext::Node(next) => {
              // Current node is neither the first nor the last
              let old_next_prev = next.pos.as_ref().unwrap().prev.replace(NodePrev::Node(prev));
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_next_prev.maybe_node().unwrap()).as_ptr()
                    as *const Node<T>
                );
              }
              let old_prev_next = prev.pos.as_ref().unwrap().next.replace(NodeNext::Node(next));
              unsafe {
                assert_eq!(
                  self as *const Node<T>,
                  Pin::into_inner_unchecked(old_prev_next.maybe_node().unwrap()).as_ptr()
                    as *const Node<T>
                );
              }
            }
          }
        }
      },
    }
  }
}

pub struct List<T: 'static> {
  head: Cell<Option<Pin<LifetimeErased<Node<T>>>>>,
  _0: PhantomPinned,
}

impl<T> List<T> {
  pub fn new() -> Self {
    Self {
      head: Cell::new(None),
      _0: PhantomPinned,
    }
  }
  // pub fn push(&mut self, value: T) -> Pin<&'static Node<T>> {
  //   let mut new_node = Node {
  //     value,
  //     pos: Some(Link {
  //       prev: Cell::new(NodePrev::Head()),
  //       next: Cell::new(NodeNext::Tail),
  //     }),
  //     _0: PhantomPinned,
  //   };
  //   let mut foo = Pin::new(ManuallyDrop::new(new_node));

  //   self.head.replace(Some())
  // }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    loop {
      match self.head.take() {
        None => break,
        Some(x) => {
          // SAFETY: The returned pointer must not moved out of until its contents are dropped. (It will be dropped immediately.)
          let x = unsafe { Pin::into_inner_unchecked(x) }.as_ptr();
          // SAFETY: The list invariants ensure that this pointer is valid and droppable.
          unsafe { core::ptr::drop_in_place(x) };
        }
      }
    }
  }
}

fn refcell_borrow_pinned<T>(x: Pin<core::cell::RefCell<T>>) {
  
}

// make a refcell<node<T>>; link is blank
// insert into list with &mut refcell<node<T>>
// node must be pinned to ensure drop is run


// client creates node
// node inserted
// node removed on drop
// list must ensure that client will drop node
// therefore list must only insert when the node is pinned
// which means the client must be able to get a "pinned node"
// so a pin to a pointer to the node
// problem: lifetimes
// Pin<&'a mut Node> can't be stuck into a Link, because it has the wrong lifetime
// lifetime must be erased, which means raw pointers
// pointers can be null, dangling, misaligned, or valid
// we can exclude null with NonNull
// we can exclude misaligned by making them from valid references
// to exclude dangling, we must ensure that the object pointed to is still alive and hasn't been moved
// that means the object must be pinned -- except we want a Pin<*const T> instead of a Pin<&T>, which isn't a thing b/c *const T doesn't have Deref
// so we need a type of lifetime-erased pointer, which implements Deref
// deref is safe, so we have to ensure that the lifetime-erased pointer will always be safe to deref
// we can ensure it's not null by using NonNull
// we can exclude misalignment by creating it from a reference
// to exclude dangling, we must create it from a Pin<&T>
// problem: we must ensure the object is never mutated, because lifetime erasure means we're removing the borrow checker's ability to ensure that for us
// solution: make a wrapper object which ensures its contents are always readonly
// problem: we can't make our Pin<*const T> replacement out of Pin<Readonly<T>> because it owns the data -- and the Pin isn't pinned itself
// we could avoid moves by making the data non-Clone but then mem::forget(Pin<Readonly<T>>) means no destructor runs
// so we need a ReadonlyRef... can we use &Readonly<T>?
// we'd get Pin<&Readonly<T>>
// we want to make a Pin<*const T> equivalent, and we need to know that the lifetime-erased pointer is always safe to deref
// so we need to know it's pinned and readonly
// Readonly