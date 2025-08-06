use std::path::PathBuf;

use bon::Builder;

use crate::common::OnOff;
use crate::to_command::{ToArg, ToCommand};

pub enum Shift {
    N(usize),
    Auto,
}

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
#[derive(Builder)]
pub struct Icount {
    shift: Option<Shift>,
    align: Option<OnOff>,
    sleep: Option<OnOff>,
    rr: Option<RecordReplay>,
    rrfile: Option<PathBuf>,
    rrsnapshot: Option<String>,
}

impl ToCommand for Icount {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-icount".to_string());

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
        cmd.push(args.join(","));
        cmd
    }
}
