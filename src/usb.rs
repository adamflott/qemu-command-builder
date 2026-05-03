use crate::to_command::ToCommand;
use proptest_derive::Arbitrary;
use std::str::FromStr;

pub(crate) const ARG_USBDEVICE: &str = "-usbdevice";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum USBDevice {
    Braille,
    Keyboard,
    Mouse,
    Tablet,
    WacomTablet,
}

impl ToCommand for USBDevice {
    fn command(&self) -> String {
        ARG_USBDEVICE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            USBDevice::Braille => args.push("braille".to_string()),
            USBDevice::Keyboard => args.push("keyboard".to_string()),
            USBDevice::Mouse => args.push("mouse".to_string()),
            USBDevice::Tablet => args.push("tablet".to_string()),
            USBDevice::WacomTablet => args.push("wacom-tablet".to_string()),
        }
        args
    }
}

impl FromStr for USBDevice {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
