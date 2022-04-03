use usb_device::class_prelude::*;
use usb_device::Result;
use usb_device::descriptor;

pub struct U2FInterface<'a, B: UsbBus> {
  pub interface_name: &'a str,
  pub interface_num: InterfaceNumber,
  interface_str: StringIndex,
  ep_interrupt_in: EndpointIn<'a, B>,
  ep_interrupt_out: EndpointOut<'a, B>,
  interrupt_buf: [u8; 64],
  expect_interrupt_in_complete: bool,
  expect_interrupt_out: bool,
}

impl<B: UsbBus> U2FInterface<'_, B> {
  pub fn new<'a>(alloc: &'a UsbBusAllocator<B>, interface_name: &'a str) -> U2FInterface<'a, B> {
    U2FInterface {
      interface_name: interface_name,
      interface_str: alloc.string(),
      interface_num: alloc.interface(),
      ep_interrupt_in: alloc.interrupt(64, 1),
      ep_interrupt_out: alloc.interrupt(64, 1),
      interrupt_buf: [0; 64],
      expect_interrupt_in_complete: false,
      expect_interrupt_out: false,
    }
  }

  /// Must be called after polling the UsbDevice.
  pub fn poll(&mut self) {
    match self.ep_interrupt_out.read(&mut self.interrupt_buf) {
      Ok(count) => {
        if self.expect_interrupt_out {
          self.expect_interrupt_out = false;
        } else {
          panic!("unexpectedly read data from interrupt out endpoint");
        }
        // match (self.callback)(&self.interrupt_buf[0..count]) {
        //   Some(outBuf) => {
        //     self.ep_interrupt_in.write(outBuf).expect("interrupt write");
        //     self.expect_interrupt_in_complete = true;
        //   }
        //   None => ()
        // }
        self.ep_interrupt_in.write(&self.interrupt_buf[0..count]).expect("interrupt write");
        self.expect_interrupt_in_complete = true;
      },
      Err(UsbError::WouldBlock) => { },
      Err(err) => panic!("interrupt read {:?}", err),
    };
  }
}

impl<B: UsbBus> UsbClass<B> for U2FInterface<'_, B> {
  fn reset(&mut self) {
    self.expect_interrupt_in_complete = false;
    self.expect_interrupt_out = false;
  }

  fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
    writer.interface_alt(self.interface_num, 0, 0x03, 0x00, 0x00, Some(self.interface_str))?;
    writer.write(0x21, &[
      // HID Class spec version 1.11
      0x11,
      0x01,
      // Country code not supported
      0x00,
      // Number of following descriptors
      1,
      // We have a HID report descriptor the host should read
      0x22,
      // HID report descriptor size,
      0x22,
      0x00,
      // (self.report_descriptor.len() & 0xFF) as u8,
      // (self.report_descriptor.len() >> 8 & 0xFF) as u8,
    ])?;
    writer.endpoint(&self.ep_interrupt_in)?;
    writer.endpoint(&self.ep_interrupt_out)?;

    Ok(())
  }

  fn get_string(&self, index: StringIndex, lang_id: u16) -> Option<&str> {
    if lang_id == descriptor::lang_id::ENGLISH_US {
      if index == self.interface_str {
        return Some(self.interface_name);
      }
    }

    None
  }

  fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
    if addr == self.ep_interrupt_in.address() {
      if self.expect_interrupt_in_complete {
        self.expect_interrupt_in_complete = false;
      } else {
        panic!("unexpected endpoint_in_complete");
      }
    }
  }

  fn endpoint_out(&mut self, addr: EndpointAddress) {
    if addr == self.ep_interrupt_out.address() {
      self.expect_interrupt_out = true;
    }
  }

  // Handle control requests to the host.
  fn control_in(&mut self, xfer: ControlIn<B>) {
    let req = xfer.request();

    // Bail out if its not relevant to our interface.
    if req.index != u8::from(self.interface_num) as u16 {
      return;
    }

    match (req.request_type, req.request) {
        (control::RequestType::Standard, control::Request::GET_DESCRIPTOR) => {
          match (req.value >> 8) as u8 {
            0x22 => {
              xfer.accept_with_static(&[
                0x06, 0xd0, 0xf1, 0x09, 0x01, 0xa1, 0x01, 0x09, 0x20,
                0x15, 0x00, 0x26, 0xff, 0x00, 0x75, 0x08, 0x95, 0x40,
                0x81, 0x02, 0x09, 0x21, 0x15, 0x00, 0x26, 0xff, 0x00,
                0x75, 0x08, 0x95, 0x40, 0x91, 0x02, 0xc0
              ]).ok();
            }
            0x21 => {
              let buf = &[
                // Length of buf inclusive of size prefix
                9,
                // Descriptor type
                0x21,
                // HID Class spec version 1.11
                0x11,
                0x01,
                // Country code not supported
                0x00,
                // Number of following descriptors
                1,
                // We have a HID report descriptor the host should read
                0x22,
                // HID report descriptor size,
                0x22,
                0x00,
              ];
              xfer.accept_with(buf).ok();
            }
            _ => {}
          }
        }
        (control::RequestType::Class, 0x01) => {
          xfer.reject().ok(); // Not supported for now
        }
        (control::RequestType::Class, 0x02) => {
          xfer.reject().ok(); // Not supported for now
        }
        _ => {}
    }
  }
}
