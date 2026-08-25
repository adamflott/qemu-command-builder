use crate::parsers::{ARG_NET, ARG_NIC, DELIM_COMMA};
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

/// An order-preserving property used by the broad `-nic` and legacy `-net` syntaxes.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub struct NetworkProperty {
    pub key: String,
    pub value: Option<String>,
}

/// QEMU's shortcut for creating a host backend and an on-board NIC together.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Nic {
    backend: String,
    #[builder(default)]
    properties: Vec<NetworkProperty>,
}

/// The compatibility `-net` interface retained by QEMU 11.1.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct LegacyNet {
    kind: String,
    #[builder(default)]
    properties: Vec<NetworkProperty>,
}

macro_rules! impl_network_option {
    ($type:ty, $field:ident, $arg:expr) => {
        impl ToCommand for $type {
            fn command(&self) -> String {
                $arg.to_string()
            }

            fn to_args(&self) -> Vec<String> {
                let mut parts = vec![self.$field.clone()];
                for property in &self.properties {
                    parts.push(match &property.value {
                        Some(value) => format!("{}={}", property.key, value),
                        None => property.key.clone(),
                    });
                }
                vec![parts.join(DELIM_COMMA)]
            }
        }

        impl FromStr for $type {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let mut parts = value.split(DELIM_COMMA);
                let first = parts.next().filter(|part| !part.is_empty()).ok_or_else(|| format!("empty {} argument", $arg))?;
                let properties = parts
                    .map(|part| match part.split_once('=') {
                        Some((key, value)) if !key.is_empty() => Ok(NetworkProperty {
                            key: key.to_string(),
                            value: Some(value.to_string()),
                        }),
                        None if !part.is_empty() => Ok(NetworkProperty { key: part.to_string(), value: None }),
                        _ => Err(format!("invalid {} property: {part}", $arg)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Self {
                    $field: first.to_string(),
                    properties,
                })
            }
        }
    };
}

impl_network_option!(Nic, backend, ARG_NIC);
impl_network_option!(LegacyNet, kind, ARG_NET);
