use crate::parsers::DELIM_COMMA;
use crate::shell_string::{ShellString, ShellStringError};
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::collections::BTreeMap;
use std::str::FromStr;
use winnow::Result;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{opt, separated, separated_pair};
use winnow::prelude::*;
use winnow::token::literal;

pub(crate) const ARG_DEVICE: &str = "-device";

///Add device driver. prop=value sets driver properties. Valid
/// properties depend on the driver. To get help on possible drivers and
/// properties, use ``-device help`` and ``-device driver,help``.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Device {
    device: ShellString,
    properties: BTreeMap<ShellString, ShellString>,
}

impl Device {
    pub fn new<S: AsRef<str>>(device: S) -> Self {
        Device {
            device: ShellString { s: device.as_ref().to_string() },
            properties: Default::default(),
        }
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties.insert(ShellString { s: key.as_ref().to_string() }, ShellString { s: value.as_ref().to_string() });
        self
    }
}

impl ToCommand for Device {
    fn command(&self) -> String {
        ARG_DEVICE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.device.s.clone()];

        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Device {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        device.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

// TODO move
fn parse_key_value_pair<'a>(s: &mut &'a str) -> ModalResult<(&'a str, &'a str)> {
    let (q, r) = separated_pair(alphanumeric1, "=", alphanumeric1).parse_next(s)?;
    Ok((q, r))
}

fn props(s: &mut &str) -> ModalResult<BTreeMap<ShellString, ShellString>> {
    let _ = literal(DELIM_COMMA).parse_next(s)?;
    let ps: Vec<(&str, &str)> = separated(1.., parse_key_value_pair, DELIM_COMMA).parse_next(s)?;
    let ps: Vec<(ShellString, ShellString)> = ps.into_iter().map(|(k, v)| (ShellString { s: k.to_string() }, ShellString { s: v.to_string() })).collect();
    let mut pt = BTreeMap::new();
    for (k, v) in ps {
        pt.insert(k, v);
    }
    Ok(pt)
}
pub fn device(s: &mut &str) -> ModalResult<Device> {
    let dev = alphanumeric1.parse_next(s)?;
    let properties = opt(props).parse_next(s)?.unwrap_or_default();
    Ok(Device {
        device: ShellString { s: dev.to_string() },
        properties,
    })
}
