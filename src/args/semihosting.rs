use crate::common::OnOff;
use crate::parsers::{ARG_SEMIHOSTING_CONFIG, DELIM_COMMA};
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum SemihostingTarget {
    Native,
    Gdb,
    Auto,
}

impl ToArg for SemihostingTarget {
    fn to_arg(&self) -> &str {
        match self {
            Self::Native => "native",
            Self::Gdb => "gdb",
            Self::Auto => "auto",
        }
    }
}

impl FromStr for SemihostingTarget {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "native" => Ok(Self::Native),
            "gdb" => Ok(Self::Gdb),
            "auto" => Ok(Self::Auto),
            _ => Err(format!("invalid semihosting target: {value}")),
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct SemihostingConfig {
    enable: Option<OnOff>,
    target: Option<SemihostingTarget>,
    chardev: Option<String>,
    userspace: Option<OnOff>,
    #[builder(default)]
    args: Vec<String>,
}

impl ToCommand for SemihostingConfig {
    fn command(&self) -> String {
        ARG_SEMIHOSTING_CONFIG.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut parts = Vec::new();
        if let Some(enable) = &self.enable {
            parts.push(format!("enable={}", enable.to_arg()));
        }
        if let Some(target) = &self.target {
            parts.push(format!("target={}", target.to_arg()));
        }
        if let Some(chardev) = &self.chardev {
            parts.push(format!("chardev={chardev}"));
        }
        if let Some(userspace) = &self.userspace {
            parts.push(format!("userspace={}", userspace.to_arg()));
        }
        parts.extend(self.args.iter().map(|arg| format!("arg={arg}")));
        vec![parts.join(DELIM_COMMA)]
    }
}

impl FromStr for SemihostingConfig {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut config = Self {
            enable: None,
            target: None,
            chardev: None,
            userspace: None,
            args: Vec::new(),
        };
        for part in value.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid semihosting-config option: {part}"))?;
            match key {
                "enable" => config.enable = Some(value.parse::<OnOff>().map_err(|_| format!("invalid enable value: {value}"))?),
                "target" => config.target = Some(value.parse()?),
                "chardev" => config.chardev = Some(value.to_string()),
                "userspace" => config.userspace = Some(value.parse::<OnOff>().map_err(|_| format!("invalid userspace value: {value}"))?),
                "arg" => config.args.push(value.to_string()),
                other => return Err(format!("unsupported semihosting-config option: {other}")),
            }
        }
        Ok(config)
    }
}
