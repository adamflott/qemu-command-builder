use crate::parsers::DELIM_COMMA;
use crate::shell_string::ShellStringError;
use crate::to_command::{ToArg, ToCommand};
use crate::{QDateTime, qao};
use bon::Builder;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use proptest_derive::Arbitrary;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub(crate) const ARG_RTC: &str = "-rtc";

const KEY_CLOCK: &str = "clock=";
const KEY_DRIFT: &str = "driftfix=";
const KEY_BASE: &str = "base=";
const KEY_BASE_UTC: &str = "utc";
const KEY_BASE_LOCALTIME: &str = "localtime";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RtcBase {
    /// Use the host UTC time.
    Utc,
    /// Use the host local time.
    Localtime,
    /// Start from a specific date/time.
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
                Err(_) => match NaiveDate::parse_from_str(maybe_dt, "%Y-%m-%d") {
                    Ok(date) => Ok(RtcBase::Datetime(QDateTime(date.and_hms_opt(0, 0, 0).unwrap().and_utc()))),
                    Err(_) => Err(()),
                },
            },
        }
    }
}
/// Supported RTC clock sources.
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

/// Supported QEMU RTC drift correction modes.
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
/// A QEMU `-rtc` definition.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Rtc {
    /// The base RTC time.
    base: Option<RtcBase>,
    /// The RTC clock source.
    clock: Option<RtcClock>,
    /// The RTC drift correction mode.
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
        let mut base = None;
        let mut clock = None;
        let mut drift_fix = None;

        for part in s.split(DELIM_COMMA).filter(|part| !part.is_empty()) {
            let (key, value) = part.split_once('=').ok_or_else(|| ShellStringError::new(format!("invalid -rtc option: {part}")))?;
            match key {
                "base" => base = Some(value.parse::<RtcBase>().map_err(|_| ShellStringError::new(format!("invalid base value: {value}")))?),
                "clock" => clock = Some(value.parse::<RtcClock>().map_err(|_| ShellStringError::new(format!("invalid clock value: {value}")))?),
                "driftfix" => {
                    drift_fix =
                        Some(value.parse::<RtcDriftFix>().map_err(|_| ShellStringError::new(format!("invalid driftfix value: {value}")))?)
                }
                "drift" => {
                    drift_fix =
                        Some(value.parse::<RtcDriftFix>().map_err(|_| ShellStringError::new(format!("invalid drift value: {value}")))?)
                }
                other => return Err(ShellStringError::new(format!("unsupported -rtc option: {other}"))),
            }
        }

        Ok(Rtc { base, clock, drift_fix })
    }
}
