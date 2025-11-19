use std::str::FromStr;

use proptest_derive::Arbitrary;
use winnow::Result;
use winnow::combinator::{alt, fail};
use winnow::prelude::*;
use winnow::token::literal;

use crate::to_command::{ToArg, ToCommand};

pub(crate) const ARG_ACTION: &str = "-action";

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

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Action {
    Reboot(RebootAction),
    Shutdown(ShutdownAction),
    Panic(PanicAction),
    Watchdog(WatchdogAction),
}

impl ToCommand for Action {
    fn command(&self) -> String {
        ARG_ACTION.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        match self {
            Action::Reboot(action) => {
                args.push(format!("{}{}", KEY_REBOOT, action.to_arg()));
            }
            Action::Shutdown(action) => {
                args.push(format!("{}{}", KEY_SHUTDOWN, action.to_arg()));
            }
            Action::Panic(action) => {
                args.push(format!("{}{}", KEY_PANIC, action.to_arg()));
            }
            Action::Watchdog(action) => {
                args.push(format!("{}{}", KEY_WATCHDOG, action.to_arg()));
            }
        }
        args
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        action.parse(s).map_err(|e| e.to_string())
    }
}

fn reboot(s: &mut &str) -> Result<Action> {
    let _ = literal(KEY_REBOOT).parse_next(s)?;
    let rb = alt((literal(VAL_RESET), literal(VAL_SHUTDOWN))).parse_next(s)?;
    match rb {
        VAL_RESET => Ok(Action::Reboot(RebootAction::Reset)),
        VAL_SHUTDOWN => Ok(Action::Reboot(RebootAction::Shutdown)),
        _ => fail(s),
    }
}

fn shutdown(s: &mut &str) -> Result<Action> {
    let _ = literal(KEY_SHUTDOWN).parse_next(s)?;
    let rb = alt((literal(VAL_POWEROFF), literal(VAL_PAUSE))).parse_next(s)?;
    match rb {
        VAL_POWEROFF => Ok(Action::Shutdown(ShutdownAction::PowerOff)),
        VAL_PAUSE => Ok(Action::Shutdown(ShutdownAction::Pause)),
        _ => fail(s),
    }
}

fn panic(s: &mut &str) -> Result<Action> {
    let _ = literal(KEY_PANIC).parse_next(s)?;
    let rb = alt((literal(VAL_PAUSE), literal(VAL_SHUTDOWN), literal(VAL_EXIT_FAILURE), literal(VAL_NONE))).parse_next(s)?;
    match rb {
        VAL_PAUSE => Ok(Action::Panic(PanicAction::Pause)),
        VAL_SHUTDOWN => Ok(Action::Panic(PanicAction::Shutdown)),
        VAL_EXIT_FAILURE => Ok(Action::Panic(PanicAction::ExitFailure)),
        VAL_NONE => Ok(Action::Panic(PanicAction::None)),
        _ => fail(s),
    }
}

fn watchdog(s: &mut &str) -> Result<Action> {
    let _ = literal(KEY_WATCHDOG).parse_next(s)?;
    let rb = alt((
        literal(VAL_RESET),
        literal(VAL_SHUTDOWN),
        literal(VAL_POWEROFF),
        literal(VAL_INJECT_NMI),
        literal(VAL_PAUSE),
        literal(VAL_DEBUG),
        literal(VAL_NONE),
    ))
    .parse_next(s)?;
    match rb {
        VAL_RESET => Ok(Action::Watchdog(WatchdogAction::Reset)),
        VAL_SHUTDOWN => Ok(Action::Watchdog(WatchdogAction::Shutdown)),
        VAL_POWEROFF => Ok(Action::Watchdog(WatchdogAction::PowerOff)),
        VAL_INJECT_NMI => Ok(Action::Watchdog(WatchdogAction::InjectNmi)),
        VAL_PAUSE => Ok(Action::Watchdog(WatchdogAction::Pause)),
        VAL_DEBUG => Ok(Action::Watchdog(WatchdogAction::Debug)),
        VAL_NONE => Ok(Action::Watchdog(WatchdogAction::None)),
        _ => fail(s),
    }
}

pub fn action(s: &mut &str) -> Result<Action> {
    alt((reboot, shutdown, panic, watchdog)).parse_next(s)
}
