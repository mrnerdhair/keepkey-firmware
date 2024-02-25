use futures::*;
use state_machine_future::*;

mod timer;

// #[derive(StateMachineFuture)]
// enum HomeStateMachine {
//   #[state_machine_future(start, transitions(HomeAnimating, Screensaver, Error))]
//   Home,
//   #[state_machine_future(transitions(Home, Info))]
//   HomeAnimating,
//   #[state_machine_future(transitions(Home, HomeAnimating))]
//   Screensaver,
//   #[state_machine_future(transitions(Screensaver, InfoAnimating))]
//   Info,
//   #[state_machine_future(transitions(Info, Home))]
//   InfoAnimating,

//   #[state_machine_future(ready)]
//   Ready(()),

//   #[state_machine_future(error)]
//   Error(core::convert::Infallible),
// }

