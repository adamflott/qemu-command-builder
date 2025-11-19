use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};
use proptest_derive::Arbitrary;
use std::str::FromStr;

pub(crate) const ARG_OVERCOMMIT: &str = "-overcommit";

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
                args.push(memlock.to_arg().to_string());
            }
            Overcommit::CpuPm(cpupm) => {
                args.push(cpupm.to_arg().to_string());
            }
        }

        args
    }
}

impl FromStr for Overcommit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
