use pretty_assertions::assert_eq;
use qemu_command_builder::action::{Action, PanicAction, RebootAction, ShutdownAction, WatchdogAction};
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn action_displays_single_event() {
    let action = Action::builder().panic(PanicAction::None).build();

    assert_eq!("panic=none", action.to_single_arg());
}

#[test]
fn action_round_trips_multiple_events() {
    let action = Action::builder().reboot(RebootAction::Shutdown).shutdown(ShutdownAction::Pause).watchdog(WatchdogAction::Debug).build();

    let rendered = action.to_single_arg();

    assert_eq!("reboot=shutdown,shutdown=pause,watchdog=debug", rendered);
    assert_eq!(action, Action::from_str(&rendered).unwrap());
}

#[test]
fn action_parses_mixed_order_into_canonical_output() {
    let parsed = Action::from_str("watchdog=pause,reboot=shutdown,panic=exit-failure").unwrap();

    assert_eq!("reboot=shutdown,panic=exit-failure,watchdog=pause", parsed.to_single_arg());
    assert_eq!(parsed, Action::from_str(&parsed.to_single_arg()).unwrap());
}
