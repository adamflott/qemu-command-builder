use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_ICOUNT: &str = "-icount";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Shift {
    N(usize),
    Auto,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RecordReplay {
    Record,
    Replay,
}

impl ToArg for RecordReplay {
    fn to_arg(&self) -> &str {
        match self {
            RecordReplay::Record => "record",
            RecordReplay::Replay => "replay",
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Icount {
    shift: Option<Shift>,
    align: Option<OnOff>,
    sleep: Option<OnOff>,
    rr: Option<RecordReplay>,
    rrfile: Option<PathBuf>,
    rrsnapshot: Option<String>,
}

impl ToCommand for Icount {
    fn command(&self) -> String {
        ARG_ICOUNT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(shift) = &self.shift {
            match shift {
                Shift::N(size) => {
                    args.push(format!("shift={}", size));
                }
                Shift::Auto => {
                    args.push("shift=auto".to_string());
                }
            }
        }
        if let Some(align) = &self.align {
            args.push(format!("align={}", align.to_arg()));
        }
        if let Some(sleep) = &self.sleep {
            args.push(format!("sleep={}", sleep.to_arg()));
        }
        if let Some(rr) = &self.rr {
            args.push(format!("rr={}", rr.to_arg()));
        }
        if let Some(rrfile) = &self.rrfile {
            args.push(format!("rrfile={}", rrfile.display()));
        }
        if let Some(rrsnapshot) = &self.rrsnapshot {
            args.push(format!("rrsnapshot={}", rrsnapshot));
        }

        args
    }
}

impl FromStr for Icount {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
