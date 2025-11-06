use crate::to_command::{ToArg, ToCommand};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum RebootAction {
    #[default]
    Reset,
    Shutdown,
}

impl ToArg for RebootAction {
    fn to_arg(&self) -> &str {
        match self {
            RebootAction::Reset => "reset",
            RebootAction::Shutdown => "shutdown",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ShutdownAction {
    #[default]
    PowerOff,
    Pause,
}

impl ToArg for ShutdownAction {
    fn to_arg(&self) -> &str {
        match self {
            ShutdownAction::PowerOff => "poweroff",
            ShutdownAction::Pause => "pause",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
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
            PanicAction::Pause => "pause",
            PanicAction::Shutdown => "shutdown",
            PanicAction::ExitFailure => "exit-failure",
            PanicAction::None => "none",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
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
            WatchdogAction::Reset => "reset",
            WatchdogAction::Shutdown => "shutdown",
            WatchdogAction::PowerOff => "poweroff",
            WatchdogAction::InjectNmi => "inject-nmi",
            WatchdogAction::Pause => "pause",
            WatchdogAction::Debug => "debug",
            WatchdogAction::None => "none",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Action {
    Reboot(RebootAction),
    Shutdown(ShutdownAction),
    Panic(PanicAction),
    Watchdog(WatchdogAction),
}

impl ToCommand for Action {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-action".to_string());

        match self {
            Action::Reboot(action) => {
                cmd.push(format!("reboot={}", action.to_arg()));
            }
            Action::Shutdown(action) => {
                cmd.push(format!("shutdown={}", action.to_arg()));
            }
            Action::Panic(action) => {
                cmd.push(format!("panic={}", action.to_arg()));
            }
            Action::Watchdog(action) => {
                cmd.push(format!("watchdog={}", action.to_arg()));
            }
        }

        cmd
    }
}
