use usb_device::class_prelude::*;

const MS_OS_STRING_DESCRIPTOR_INDEX: u8 = 0xEE;
const MS_OS_FEATURE_DESCRIPTOR_EXTENDED_COMPAT_ID: u16 = 0x0004;
const MS_OS_FEATURE_DESCRIPTOR_EXTENDED_PROPERTIES: u16 = 0x0005;

#[repr(u16)]
enum MsOsFeatureDescriptor {
  Genre = 0x0001,
  ExtendedCompatId = 0x0004,
  ExtendedProperties = 0x0005,
}

pub struct WinUsb {
  interface_num: u8,
  os_string_descriptor: [u8; 8],
  emulate_bugs: bool,
}

impl WinUsb {
  pub fn new<B: UsbBus>(_alloc: &UsbBusAllocator<B>, interface_num: u8, vendor_code: u8, emulate_bugs: bool) -> WinUsb {
    WinUsb {
      interface_num,
      os_string_descriptor: ['M' as u8, 'S' as u8, 'F' as u8, 'T' as u8, '1' as u8, '0' as u8, '0' as u8, vendor_code],
      emulate_bugs,
    }
  }
}

impl<B: UsbBus> UsbClass<B> for WinUsb {
  fn get_string(&self, index: StringIndex, lang_id: u16) -> Option<&str> {
    if !self.emulate_bugs && u8::from(index) == MS_OS_STRING_DESCRIPTOR_INDEX && lang_id == 0 {
      return Some(core::str::from_utf8(&self.os_string_descriptor).unwrap())
    }

    None
  }

  fn control_in(&mut self, xfer: ControlIn<B>) {
    let req = xfer.request();
    let _interface_num: u8 = (req.value >> 8) as u8;
    let page_num: u8 = (req.value & 0xFF) as u8;

    if !(
      // Only handle requests for MS OS Descriptors
      req.direction == usb_device::UsbDirection::In &&
      req.request_type == usb_device::control::RequestType::Vendor &&
      req.request == self.os_string_descriptor[7]
    ) {
      return;
    }

    if !(
      /*
       * NOTE: Per the Microsoft Extended Compat ID OS Feature Descriptor Specification: "Because a device can have only
       * one extended compat ID descriptor, it should ignore InterfaceNumber, regardless of the value, and simply return
       * the descriptor."
       */
      // interface_num == self.interface_num &&
      // None of the descriptors we return are large (>64KiB), so a request for anything other than the first page is an error.
      page_num == 0
    ){
      xfer.reject().unwrap();
      return;
    }

    match req.index {
      MS_OS_FEATURE_DESCRIPTOR_EXTENDED_COMPAT_ID => {
        if req.recipient != usb_device::control::Recipient::Device {
          xfer.reject().unwrap();
          return;
        }
        let resp: &mut [u8] = &mut [
          0x28, 0x00, 0x00, 0x00, 0x00, 0x01, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x01, 0x57, 0x49, 0x4E, 0x55, 0x53, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        resp[16] = self.interface_num;
        xfer.accept_with(resp).unwrap();
      },
      MS_OS_FEATURE_DESCRIPTOR_EXTENDED_PROPERTIES => {
        /*
         * NOTE: The official MS doc says that these requests will always be targeted at the Device, not the Interface.
         * However, https://github.com/pbatard/libwdi/wiki/WCID-Devices#Implementation indicates that this in in error, and
         * requests for the Extended Properties Descriptor will be targeted at the Interface instead.
         */
        if req.recipient != usb_device::control::Recipient::Interface {
          xfer.reject().unwrap();
          return;
        }
        xfer.accept_with_static(&[
          0x92, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x00, 0x01, 0x00,
          0x88, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x2a, 0x00,
          0x44, 0x00, 0x65, 0x00, 0x76, 0x00, 0x69, 0x00, 0x63, 0x00, 0x65, 0x00, 0x49, 0x00, 0x6E, 0x00, 0x74, 0x00, 0x65, 0x00, 0x72, 0x00, 0x66, 0x00, 0x61, 0x00, 0x63, 0x00, 0x65, 0x00, 0x47, 0x00, 0x55, 0x00, 0x49, 0x00, 0x44, 0x00, 0x73, 0x00, 0x00, 0x00,
          0x50, 0x00, 0x00, 0x00,
          0x7B, 0x00, 0x30, 0x00, 0x32, 0x00, 0x36, 0x00, 0x33, 0x00, 0x62, 0x00, 0x35, 0x00, 0x31, 0x00, 0x32, 0x00, 0x2D, 0x00, 0x38, 0x00, 0x38, 0x00, 0x63, 0x00, 0x62, 0x00, 0x2D, 0x00, 0x34, 0x00, 0x31, 0x00, 0x33, 0x00, 0x36, 0x00, 0x2D, 0x00, 0x39, 0x00, 0x36, 0x00, 0x31, 0x00, 0x33, 0x00, 0x2D, 0x00, 0x35, 0x00, 0x63, 0x00, 0x38, 0x00, 0x65, 0x00, 0x31, 0x00, 0x30, 0x00, 0x39, 0x00, 0x64, 0x00, 0x38, 0x00, 0x65, 0x00, 0x66, 0x00, 0x35, 0x00, 0x7D, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]).unwrap();
      },
      _ => xfer.reject().unwrap(),
    }
  }
}
