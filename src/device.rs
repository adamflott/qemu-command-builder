use bon::Builder;

use crate::to_command::ToCommand;

///Add device driver. prop=value sets driver properties. Valid
/// properties depend on the driver. To get help on possible drivers and
/// properties, use ``-device help`` and ``-device driver,help``.
///
/// TODO
/// - constrain
#[derive(Default, Builder)]
pub struct Device {
    device: String,
    properties: Vec<(String, String)>,
}

impl Device {
    pub fn new<S: AsRef<str>>(device: S) -> Self {
        Device {
            device: device.as_ref().to_string(),
            properties: Default::default(),
        }
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties
            .push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
}
impl ToCommand for Device {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-device".to_string());
        let mut args = vec![self.device.clone()];

        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }
        cmd.push(args.join(","));

        cmd
    }
}
