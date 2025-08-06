use crate::to_command::ToCommand;

pub enum USBDevice {
    Braille,
    Keyboard,
    Mouse,
    Tablet,
    WacomTablet,
}
impl ToCommand for USBDevice {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-usbdevice".to_string());

        match self {
            USBDevice::Braille => cmd.push("braille".to_string()),
            USBDevice::Keyboard => cmd.push("keyboard".to_string()),
            USBDevice::Mouse => cmd.push("mouse".to_string()),
            USBDevice::Tablet => cmd.push("tablet".to_string()),
            USBDevice::WacomTablet => cmd.push("wacom-tablet".to_string()),
        }
        cmd
    }
}
