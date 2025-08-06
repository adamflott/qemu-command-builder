use bon::Builder;
use chrono::{DateTime, Utc};

use crate::to_command::{ToArg, ToCommand};

pub enum RtcBase {
    Utc,
    Localtime,
    Datetime(DateTime<Utc>),
}

pub enum RtcClock {
    Host,
    Rt,
    Vm,
}

impl ToArg for RtcClock {
    fn to_arg(&self) -> &str {
        match self {
            RtcClock::Host => "host",
            RtcClock::Rt => "rt",
            RtcClock::Vm => "vm",
        }
    }
}
pub enum RtcDriftFix {
    None,
    Slew,
}

impl ToArg for RtcDriftFix {
    fn to_arg(&self) -> &str {
        match self {
            RtcDriftFix::None => "none",
            RtcDriftFix::Slew => "slew",
        }
    }
}
#[derive(Default, Builder)]
pub struct Rtc {
    base: Option<RtcBase>,
    clock: Option<RtcClock>,
    drift_fix: Option<RtcDriftFix>,
}

impl ToCommand for Rtc {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-rtc".to_string());

        let mut args = vec![];

        if let Some(base) = &self.base {
            match base {
                RtcBase::Utc => {
                    args.push("base=utc".to_string());
                }
                RtcBase::Localtime => {
                    args.push("base=localtime".to_string());
                }
                RtcBase::Datetime(dt) => {
                    // use formats 2006-06-17T16:01:21 or 2006-06-17
                    // TODO support date based
                    let formatted = format!("base={}", dt.format("%Y-%m-%dT%H:%M:%S"));
                    args.push(formatted);
                }
            }
        }

        if let Some(clock) = &self.clock {
            args.push(format!("clock={}", clock.to_arg()));
        }
        if let Some(drift_fix) = &self.drift_fix {
            args.push(format!("drift={}", drift_fix.to_arg()));
        }

        cmd.push(args.join(","));
        cmd
    }
}
