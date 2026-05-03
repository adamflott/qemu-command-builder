use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::{ToArg, ToCommand};
use crate::{QDateTime, pco0, qao};
use bon::Builder;
use chrono::NaiveDateTime;
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use winnow::Result;
use winnow::ascii::alphanumeric1;
use winnow::combinator::opt;
use winnow::prelude::*;
use winnow::token::{literal, take_while};

pub(crate) const ARG_RTC: &str = "-rtc";

const KEY_CLOCK: &str = "clock=";
const KEY_DRIFT: &str = "drift=";
const KEY_BASE: &str = "base=";
const KEY_BASE_UTC: &str = "utc";
const KEY_BASE_LOCALTIME: &str = "localtime";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RtcBase {
    Utc,
    Localtime,
    Datetime(QDateTime),
}

impl Display for RtcBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RtcBase::Utc => {
                write!(f, "{}{}", KEY_BASE, KEY_BASE_UTC)
            }
            RtcBase::Localtime => {
                write!(f, "{}{}", KEY_BASE, KEY_BASE_LOCALTIME)
            }
            RtcBase::Datetime(dt) => {
                let formatted = format!("{}{}", KEY_BASE, dt.0.format("%Y-%m-%dT%H:%M:%S"));
                write!(f, "{}", formatted)
            }
        }
    }
}

impl FromStr for RtcBase {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            KEY_BASE_UTC => Ok(RtcBase::Utc),
            KEY_BASE_LOCALTIME => Ok(RtcBase::Localtime),
            maybe_dt => match NaiveDateTime::parse_from_str(maybe_dt, "%Y-%m-%dT%H:%M:%S") {
                Ok(dt) => Ok(RtcBase::Datetime(QDateTime(dt.and_utc()))),
                Err(err) => {
                    eprintln!("{} => {}", maybe_dt, err);
                    Err(())
                }
            },
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RtcClock {
    Host,
    Rt,
    Vm,
}

impl Display for RtcClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RtcClock::Host => write!(f, "host"),
            RtcClock::Rt => write!(f, "rt"),
            RtcClock::Vm => write!(f, "vm"),
        }
    }
}

impl FromStr for RtcClock {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "host" => Ok(RtcClock::Host),
            "rt" => Ok(RtcClock::Rt),
            "vm" => Ok(RtcClock::Vm),
            _ => Err(()),
        }
    }
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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RtcDriftFix {
    None,
    Slew,
}

impl Display for RtcDriftFix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RtcDriftFix::None => write!(f, "none"),
            RtcDriftFix::Slew => write!(f, "slew"),
        }
    }
}

impl FromStr for RtcDriftFix {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(RtcDriftFix::None),
            "slew" => Ok(RtcDriftFix::Slew),
            _ => Err(()),
        }
    }
}
impl ToArg for RtcDriftFix {
    fn to_arg(&self) -> &str {
        match self {
            RtcDriftFix::None => "none",
            RtcDriftFix::Slew => "slew",
        }
    }
}
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Rtc {
    base: Option<RtcBase>,
    clock: Option<RtcClock>,
    drift_fix: Option<RtcDriftFix>,
}

impl ToCommand for Rtc {
    fn has_args(&self) -> bool {
        self.base.is_some() || self.clock.is_some() || self.drift_fix.is_some()
    }
    fn command(&self) -> String {
        ARG_RTC.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        if let Some(base) = &self.base {
            args.push(base.to_string());
        }
        qao!(&self.clock, args, KEY_CLOCK);
        qao!(&self.drift_fix, args, KEY_DRIFT);

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Rtc {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rtc.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

fn parse_date_time<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == ':').parse_next(input)
}

pco0!(base, parse_date_time, RtcBase, KEY_BASE);
pco0!(clock, alphanumeric1, RtcClock, KEY_CLOCK);
pco0!(drift_fix, alphanumeric1, RtcDriftFix, KEY_DRIFT);

pub fn rtc(s: &mut &str) -> ModalResult<Rtc> {
    let base = opt(base).parse_next(s)?;
    let clock = opt(clock).parse_next(s)?;
    let drift_fix = opt(drift_fix).parse_next(s)?;
    Ok(Rtc { base, clock, drift_fix })
}
