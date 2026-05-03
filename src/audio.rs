use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;

pub(crate) const ARG_AUDIO: &str = "-audio";

const KEY_DRIVER: &str = "driver=";
const KEY_MODEL: &str = "model=";

/// A generic `-audio` property rendered after `driver=` and `model=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct AudioProperty {
    key: String,
    value: Option<String>,
}

impl AudioProperty {
    /// Creates a `key=value` property.
    pub fn with_value(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
        }
    }

    /// Creates a bare `key` property with no explicit value.
    pub fn flag(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }
}

/// QEMU `-audio [driver=]driver[,model=value][,prop[=value][,...]]`.
///
/// This shortcut configures a default host audio backend and optionally the
/// guest audio device model. Additional backend properties are emitted after
/// the canonical `driver` and `model` fields.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Audio {
    driver: String,
    model: Option<String>,
    #[builder(default)]
    properties: Vec<AudioProperty>,
}

impl Audio {
    /// Creates an audio configuration for the given backend driver.
    pub fn new(driver: impl Into<String>) -> Self {
        Self {
            driver: driver.into(),
            model: None,
            properties: Vec::new(),
        }
    }

    /// Sets the guest audio device model used by the `-audio` shortcut.
    pub fn model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = Some(model.into());
        self
    }

    /// Adds a `key=value` backend property.
    pub fn add_prop<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) -> &mut Self {
        self.properties.push(AudioProperty::with_value(key.as_ref(), value.as_ref()));
        self
    }

    /// Adds a bare backend property with no explicit value.
    pub fn add_flag<K: AsRef<str>>(&mut self, key: K) -> &mut Self {
        self.properties.push(AudioProperty::flag(key.as_ref()));
        self
    }
}

impl ToCommand for Audio {
    fn command(&self) -> String {
        ARG_AUDIO.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![format!("{}{}", KEY_DRIVER, self.driver)];

        if let Some(model) = &self.model {
            args.push(format!("{}{}", KEY_MODEL, model));
        }
        for property in &self.properties {
            match &property.value {
                Some(value) => args.push(format!("{}={}", property.key, value)),
                None => args.push(property.key.clone()),
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Audio {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let first = parts.next().ok_or_else(|| "empty audio argument".to_string())?;
        let driver = first.strip_prefix(KEY_DRIVER).unwrap_or(first).to_string();
        if driver.is_empty() {
            return Err("missing audio driver".to_string());
        }

        let mut audio = Audio::new(driver);

        for part in parts {
            match part.split_once('=') {
                Some(("model", value)) => audio.model = Some(value.to_string()),
                Some((key, value)) => audio.properties.push(AudioProperty::with_value(key, value)),
                None => audio.properties.push(AudioProperty::flag(part)),
            }
        }

        Ok(audio)
    }
}
