use crate::parsers::ARG_DEVICE;
use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellString;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::collections::BTreeMap;
use std::str::FromStr;

/// A generic `-device` property rendered after the device driver name.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct DeviceProperty {
    key: ShellString,
    value: Option<ShellString>,
}

impl DeviceProperty {
    /// Creates a `key=value` property.
    pub fn with_value(key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        Self {
            key: ShellString::from(key.as_ref()),
            value: Some(ShellString::from(value.as_ref())),
        }
    }

    /// Creates a bare `key` property.
    pub fn flag(key: impl AsRef<str>) -> Self {
        Self {
            key: ShellString::from(key.as_ref()),
            value: None,
        }
    }
}

/// Add a QEMU device driver with optional `prop` or `prop=value` settings.
///
/// Valid properties depend on the device driver. Use `-device help` or
/// `-device driver,help` in QEMU for the device-specific property list.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Device {
    device: ShellString,
    #[builder(default)]
    properties: BTreeMap<ShellString, Option<ShellString>>,
    json: Option<String>,
}

impl Device {
    /// Creates a new device argument for the given QEMU device driver.
    pub fn new<S: AsRef<str>>(device: S) -> Self {
        Device {
            device: ShellString::from(device.as_ref()),
            properties: Default::default(),
            json: None,
        }
    }

    /// Creates the QEMU 11.1 ARM SMMUv3 device. Properties such as `ril`,
    /// `ats`, `oas`, `ssidsize`, and `cmdqv` accept QEMU's `auto` value via
    /// [`Device::add_prop`].
    pub fn arm_smmuv3() -> Self {
        Self::new("arm-smmuv3")
    }
    /// Creates a device from QEMU's JSON command-line form.
    pub fn from_json(json: impl Into<String>) -> Result<Self, String> {
        let json = json.into();
        if !json.trim().starts_with('{') || !json.trim().ends_with('}') {
            return Err("-device JSON must be a JSON object".to_string());
        }
        Ok(Self {
            device: ShellString::from(""),
            properties: BTreeMap::new(),
            json: Some(json),
        })
    }

    /// Adds a `key=value` property to the device.
    pub fn add_prop<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        self.properties.insert(ShellString::from(key.as_ref()), Some(ShellString::from(value.as_ref())));
        self
    }

    /// Adds a bare `key` property to the device.
    pub fn add_flag<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        self.properties.insert(ShellString::from(key.as_ref()), None);
        self
    }
}

impl ToCommand for Device {
    fn command(&self) -> String {
        ARG_DEVICE.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        if let Some(json) = &self.json {
            return vec![json.clone()];
        }
        let mut args = vec![self.device.as_ref().to_string()];

        for (prop_key, prop_value) in &self.properties {
            match prop_value {
                Some(value) => args.push(format!("{}={}", prop_key.as_ref(), value.as_ref())),
                None => args.push(prop_key.as_ref().to_string()),
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().starts_with('{') {
            return Self::from_json(s);
        }
        let mut parts = s.split(DELIM_COMMA);
        let device = parts.next().ok_or_else(|| "empty device argument".to_string())?;
        if device.is_empty() {
            return Err("missing device driver".to_string());
        }

        let mut properties = BTreeMap::new();
        for part in parts {
            match part.split_once('=') {
                Some((key, value)) => {
                    properties.insert(ShellString::from(key), Some(ShellString::from_str(value)?));
                }
                None => {
                    properties.insert(ShellString::from(part), None);
                }
            }
        }

        Ok(Device {
            device: ShellString::from(device),
            properties,
            json: None,
        })
    }
}
