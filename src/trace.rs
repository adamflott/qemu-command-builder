use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_TRACE: &str = "-trace";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Trace {
    enable: Option<String>,
    events: Option<PathBuf>,
    file: Option<PathBuf>,
}

impl ToCommand for Trace {
    fn command(&self) -> String {
        ARG_TRACE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(enable) = &self.enable {
            args.push(format!("enable={}", enable));
        }
        if let Some(events) = &self.events {
            args.push(format!("events={}", events.display()));
        }
        if let Some(file) = &self.file {
            args.push(format!("file={}", file.display()));
        }
        args
    }
}

impl FromStr for Trace {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
