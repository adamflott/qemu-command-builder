use crate::parsers::ARG_ACTION;
use std::str::FromStr;

use bon::Builder;
use proptest_derive::Arbitrary;

use crate::parsers::DELIM_COMMA;
use crate::to_command::{ToArg, ToCommand};

const KEY_REBOOT: &str = "reboot=";
const KEY_SHUTDOWN: &str = "shutdown=";
const KEY_PANIC: &str = "panic=";
const KEY_WATCHDOG: &str = "watchdog=";

const VAL_RESET: &str = "reset";
const VAL_SHUTDOWN: &str = "shutdown";
const VAL_POWEROFF: &str = "poweroff";
const VAL_PAUSE: &str = "pause";
const VAL_EXIT_FAILURE: &str = "exit-failure";
const VAL_NONE: &str = "none";
const VAL_INJECT_NMI: &str = "inject-nmi";
const VAL_DEBUG: &str = "debug";

/// QEMU `reboot=` actions for `-action`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum RebootAction {
    #[default]
    Reset,
    Shutdown,
}

impl ToArg for RebootAction {
    fn to_arg(&self) -> &str {
        match self {
            RebootAction::Reset => VAL_RESET,
            RebootAction::Shutdown => VAL_SHUTDOWN,
        }
    }
}

impl FromStr for RebootAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            VAL_RESET => Ok(Self::Reset),
            VAL_SHUTDOWN => Ok(Self::Shutdown),
            _ => Err(format!("invalid reboot action: {s}")),
        }
    }
}

/// QEMU `shutdown=` actions for `-action`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum ShutdownAction {
    #[default]
    PowerOff,
    Pause,
}

impl ToArg for ShutdownAction {
    fn to_arg(&self) -> &str {
        match self {
            ShutdownAction::PowerOff => VAL_POWEROFF,
            ShutdownAction::Pause => VAL_PAUSE,
        }
    }
}

impl FromStr for ShutdownAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            VAL_POWEROFF => Ok(Self::PowerOff),
            VAL_PAUSE => Ok(Self::Pause),
            _ => Err(format!("invalid shutdown action: {s}")),
        }
    }
}

/// QEMU `panic=` actions for `-action`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum PanicAction {
    Pause,
    #[default]
    Shutdown,
    ExitFailure,
    None,
}

impl ToArg for PanicAction {
    fn to_arg(&self) -> &str {
        match self {
            PanicAction::Pause => VAL_PAUSE,
            PanicAction::Shutdown => VAL_SHUTDOWN,
            PanicAction::ExitFailure => VAL_EXIT_FAILURE,
            PanicAction::None => VAL_NONE,
        }
    }
}

impl FromStr for PanicAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            VAL_PAUSE => Ok(Self::Pause),
            VAL_SHUTDOWN => Ok(Self::Shutdown),
            VAL_EXIT_FAILURE => Ok(Self::ExitFailure),
            VAL_NONE => Ok(Self::None),
            _ => Err(format!("invalid panic action: {s}")),
        }
    }
}

/// QEMU `watchdog=` actions for `-action` and `-watchdog-action`.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Arbitrary)]
pub enum WatchdogAction {
    #[default]
    Reset,
    Shutdown,
    PowerOff,
    InjectNmi,
    Pause,
    Debug,
    None,
}

impl ToArg for WatchdogAction {
    fn to_arg(&self) -> &str {
        match self {
            WatchdogAction::Reset => VAL_RESET,
            WatchdogAction::Shutdown => VAL_SHUTDOWN,
            WatchdogAction::PowerOff => VAL_POWEROFF,
            WatchdogAction::InjectNmi => VAL_INJECT_NMI,
            WatchdogAction::Pause => VAL_PAUSE,
            WatchdogAction::Debug => VAL_DEBUG,
            WatchdogAction::None => VAL_NONE,
        }
    }
}

impl FromStr for WatchdogAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            VAL_RESET => Ok(Self::Reset),
            VAL_SHUTDOWN => Ok(Self::Shutdown),
            VAL_POWEROFF => Ok(Self::PowerOff),
            VAL_INJECT_NMI => Ok(Self::InjectNmi),
            VAL_PAUSE => Ok(Self::Pause),
            VAL_DEBUG => Ok(Self::Debug),
            VAL_NONE => Ok(Self::None),
            _ => Err(format!("invalid watchdog action: {s}")),
        }
    }
}

/// QEMU `-action` event handlers.
///
/// QEMU accepts a comma-separated list of event assignments such as
/// `reboot=shutdown,shutdown=pause`. This type preserves those assignments in
/// canonical field order.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Action {
    reboot: Option<RebootAction>,
    shutdown: Option<ShutdownAction>,
    panic: Option<PanicAction>,
    watchdog: Option<WatchdogAction>,
}

impl Action {
    /// Creates an empty `-action` configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToCommand for Action {
    fn has_args(&self) -> bool {
        self.reboot.is_some() || self.shutdown.is_some() || self.panic.is_some() || self.watchdog.is_some()
    }

    fn command(&self) -> String {
        ARG_ACTION.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        if let Some(action) = &self.reboot {
            args.push(format!("{}{}", KEY_REBOOT, action.to_arg()));
        }
        if let Some(action) = &self.shutdown {
            args.push(format!("{}{}", KEY_SHUTDOWN, action.to_arg()));
        }
        if let Some(action) = &self.panic {
            args.push(format!("{}{}", KEY_PANIC, action.to_arg()));
        }
        if let Some(action) = &self.watchdog {
            args.push(format!("{}{}", KEY_WATCHDOG, action.to_arg()));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut action = Action::default();

        for part in s.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid action option: {part}"))?;
            match key {
                "reboot" => action.reboot = Some(value.parse::<RebootAction>()?),
                "shutdown" => action.shutdown = Some(value.parse::<ShutdownAction>()?),
                "panic" => action.panic = Some(value.parse::<PanicAction>()?),
                "watchdog" => action.watchdog = Some(value.parse::<WatchdogAction>()?),
                other => return Err(format!("unsupported action key: {other}")),
            }
        }

        Ok(action)
    }
}
