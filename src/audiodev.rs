use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;

pub(crate) const ARG_AUDIODEV: &str = "-audiodev";

/// A generic `prop` or `prop=value` entry for `-audiodev`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct AudioDevProperty {
    /// The property name.
    pub key: String,
    /// The optional property value.
    pub value: Option<String>,
}

/// A QEMU `-audiodev [driver=]driver,id=id[,prop[=value][,...]]` definition.
///
/// This type intentionally preserves arbitrary property keys instead of
/// validating backend-specific options, so it can round-trip the generic
/// `-audiodev` surface described by QEMU.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct AudioDev {
    /// The backend driver name, such as `alsa`, `pa`, `spice`, or `wav`.
    driver: String,
    /// Generic backend/global properties in canonical output order.
    props: Vec<AudioDevProperty>,
}

impl AudioDev {
    pub fn new(driver: impl Into<String>) -> Self {
        Self {
            driver: driver.into(),
            props: Vec::new(),
        }
    }

    pub fn add_prop(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.props.push(AudioDevProperty {
            key: key.into(),
            value: Some(value.into()),
        });
        self
    }

    pub fn add_flag(&mut self, key: impl Into<String>) -> &mut Self {
        self.props.push(AudioDevProperty { key: key.into(), value: None });
        self
    }
}

impl ToCommand for AudioDev {
    fn command(&self) -> String {
        ARG_AUDIODEV.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.driver.clone()];

        let mut props = self.props.clone();
        props.sort_by(|a, b| a.key.cmp(&b.key).then_with(|| a.value.cmp(&b.value)));

        for prop in &props {
            if let Some(value) = &prop.value {
                args.push(format!("{}={}", prop.key, value));
            } else {
                args.push(prop.key.clone());
            }
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AudioDev {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let first = parts.next().ok_or_else(|| "empty -audiodev argument".to_string())?;

        let driver = if let Some(value) = first.strip_prefix("driver=") {
            value.to_string()
        } else if !first.contains('=') {
            first.to_string()
        } else {
            return Err(format!("unsupported first -audiodev component: {first}"));
        };

        let mut props = Vec::new();
        for part in parts {
            if let Some((key, value)) = part.split_once('=') {
                props.push(AudioDevProperty {
                    key: key.to_string(),
                    value: Some(value.to_string()),
                });
            } else {
                props.push(AudioDevProperty { key: part.to_string(), value: None });
            }
        }

        let has_id = props.iter().any(|prop| prop.key == "id");
        if !has_id {
            return Err("-audiodev requires id=".to_string());
        }

        Ok(Self { driver, props })
    }
}
