use crate::parsers::{ARG_OBJECT, DELIM_COMMA};
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

/// A generic QEMU `-object typename[,prop=value,...]` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Object {
    /// The QOM object type name.
    typename: String,
    /// Object properties emitted in the order stored.
    properties: Vec<(String, String)>,
    /// Raw QMP JSON form, preserved verbatim when present.
    json: Option<String>,
}

impl Object {
    pub fn new<S: AsRef<str>>(typename: S) -> Self {
        Object {
            typename: typename.as_ref().to_string(),
            properties: Default::default(),
            json: None,
        }
    }
    /// Creates an object from QEMU's JSON command-line form.
    pub fn from_json(json: impl Into<String>) -> Result<Self, String> {
        let json = json.into();
        if !json.trim().starts_with('{') || !json.trim().ends_with('}') {
            return Err("-object JSON must be a JSON object".to_string());
        }
        Ok(Self {
            typename: String::new(),
            properties: Vec::new(),
            json: Some(json),
        })
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties.push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }

    /// Creates a QEMU 11.1 HMP monitor object.
    pub fn monitor_hmp(id: impl AsRef<str>, chardev: impl AsRef<str>) -> Self {
        let mut value = Self::new("monitor-hmp");
        value.add_prop("id", id.as_ref()).add_prop("chardev", chardev.as_ref());
        value
    }

    /// Creates a QEMU 11.1 QMP monitor object.
    pub fn monitor_qmp(id: impl AsRef<str>, chardev: impl AsRef<str>) -> Self {
        let mut value = Self::new("monitor-qmp");
        value.add_prop("id", id.as_ref()).add_prop("chardev", chardev.as_ref());
        value
    }

    /// Creates an Intel TDX confidential-guest object. Nested socket addresses
    /// can be supplied with [`Object::from_json`].
    pub fn tdx_guest(id: impl AsRef<str>) -> Self {
        let mut value = Self::new("tdx-guest");
        value.add_prop("id", id.as_ref());
        value
    }

    /// Creates an IOThread object; QEMU 11.1's `poll-weight` can be added with
    /// [`Object::add_prop`].
    pub fn iothread(id: impl AsRef<str>) -> Self {
        let mut value = Self::new("iothread");
        value.add_prop("id", id.as_ref());
        value
    }
}

impl ToCommand for Object {
    fn command(&self) -> String {
        ARG_OBJECT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        if let Some(json) = &self.json {
            return vec![json.clone()];
        }
        let mut args = vec![self.typename.clone()];

        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Object {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().starts_with('{') {
            return Self::from_json(s);
        }
        let mut parts = s.split(DELIM_COMMA);
        let typename = parts.next().ok_or_else(|| "empty -object argument".to_string())?.to_string();

        let mut properties = Vec::new();
        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -object property: {part}"))?;
            properties.push((key.to_string(), value.to_string()));
        }

        Ok(Self { typename, properties, json: None })
    }
}
