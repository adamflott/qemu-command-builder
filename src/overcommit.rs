use crate::common::OnOff;
use crate::parsers::ARG_OVERCOMMIT;
use crate::to_command::{ToArg, ToCommand};
use proptest_derive::Arbitrary;
use std::str::FromStr;

/// Supported values for `mem-lock=`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum OnOffOnfault {
    On,
    Off,
    Onfault,
}

impl ToArg for OnOffOnfault {
    fn to_arg(&self) -> &str {
        match self {
            OnOffOnfault::On => "on",
            OnOffOnfault::Off => "off",
            OnOffOnfault::Onfault => "on-fault",
        }
    }
}

/// Host overcommit hints for QEMU.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Overcommit {
    MemLock(OnOffOnfault),
    CpuPm(OnOff),
}

impl ToCommand for Overcommit {
    fn command(&self) -> String {
        ARG_OVERCOMMIT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            Overcommit::MemLock(memlock) => {
                args.push(format!("mem-lock={}", memlock.to_arg()));
            }
            Overcommit::CpuPm(cpupm) => {
                args.push(format!("cpu-pm={}", cpupm.to_arg()));
            }
        }

        args
    }
}

impl FromStr for Overcommit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, value) = s.split_once('=').ok_or_else(|| format!("invalid -overcommit argument: {s}"))?;
        match key {
            "mem-lock" => match value {
                "on" => Ok(Self::MemLock(OnOffOnfault::On)),
                "off" => Ok(Self::MemLock(OnOffOnfault::Off)),
                "on-fault" => Ok(Self::MemLock(OnOffOnfault::Onfault)),
                _ => Err(format!("invalid mem-lock value: {value}")),
            },
            "cpu-pm" => Ok(Self::CpuPm(value.parse::<OnOff>().map_err(|_| format!("invalid cpu-pm value: {value}"))?)),
            other => Err(format!("unsupported -overcommit option: {other}")),
        }
    }
}
