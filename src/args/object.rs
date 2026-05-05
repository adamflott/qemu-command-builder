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
}

impl Object {
    pub fn new<S: AsRef<str>>(typename: S) -> Self {
        Object {
            typename: typename.as_ref().to_string(),
            properties: Default::default(),
        }
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties.push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
}

impl ToCommand for Object {
    fn command(&self) -> String {
        ARG_OBJECT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
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
        let mut parts = s.split(DELIM_COMMA);
        let typename = parts.next().ok_or_else(|| "empty -object argument".to_string())?.to_string();

        let mut properties = Vec::new();
        for part in parts {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -object property: {part}"))?;
            properties.push((key.to_string(), value.to_string()));
        }

        Ok(Self { typename, properties })
    }
}
