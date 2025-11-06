use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Overcommit {
    MemLock(OnOffOnfault),
    CpuPm(OnOff),
}

impl ToCommand for Overcommit {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-overcommit".to_string());
        match self {
            Overcommit::MemLock(memlock) => {
                cmd.push(memlock.to_arg().to_string());
            }
            Overcommit::CpuPm(cpupm) => {
                cmd.push(cpupm.to_arg().to_string());
            }
        }

        cmd
    }
}
