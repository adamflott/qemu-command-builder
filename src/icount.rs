use std::path::PathBuf;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::common::OnOff;
use crate::parsers::DELIM_COMMA;
use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_ICOUNT: &str = "-icount";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Shift {
    N(usize),
    Auto,
}

/// Record/replay mode for `-icount`.
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
/// A QEMU `-icount` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Icount {
    /// Virtual time shift.
    shift: Option<Shift>,
    /// Host/guest clock alignment policy.
    align: Option<OnOff>,
    /// Virtual sleep policy.
    sleep: Option<OnOff>,
    /// Record or replay mode.
    rr: Option<RecordReplay>,
    /// Record/replay log filename.
    rrfile: Option<PathBuf>,
    /// Initial snapshot name for record/replay.
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

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Icount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shift = None;
        let mut align = None;
        let mut sleep = None;
        let mut rr = None;
        let mut rrfile = None;
        let mut rrsnapshot = None;

        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid -icount option: {part}"))?;
            match key {
                "shift" => {
                    shift = Some(if value == "auto" {
                        Shift::Auto
                    } else {
                        Shift::N(value.parse::<usize>().map_err(|e| e.to_string())?)
                    })
                }
                "align" => align = Some(value.parse::<OnOff>().map_err(|_| format!("invalid align value: {value}"))?),
                "sleep" => sleep = Some(value.parse::<OnOff>().map_err(|_| format!("invalid sleep value: {value}"))?),
                "rr" => {
                    rr = Some(match value {
                        "record" => RecordReplay::Record,
                        "replay" => RecordReplay::Replay,
                        _ => return Err(format!("invalid rr value: {value}")),
                    })
                }
                "rrfile" => rrfile = Some(PathBuf::from(value)),
                "rrsnapshot" => rrsnapshot = Some(value.to_string()),
                other => return Err(format!("unsupported -icount option: {other}")),
            }
        }

        Ok(Self {
            shift,
            align,
            sleep,
            rr,
            rrfile,
            rrsnapshot,
        })
    }
}
