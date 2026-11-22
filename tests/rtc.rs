use qemu_command_builder::args::rtc::{Rtc, RtcBase};
use qemu_command_builder::to_command::ToCommand;

#[test]
fn rtc_dates_normalize_and_round_trip() {
    for (input, expected) in [
        ("utc", "utc"),
        ("localtime", "localtime"),
        ("1984-10-22T00:01:05", "1984-10-22T00:01:05"),
        ("2024-02-29", "2024-02-29T00:00:00"),
        ("2000-02-29T23:59:59", "2000-02-29T23:59:59"),
    ] {
        let base = input.parse::<RtcBase>().unwrap();
        assert_eq!(base.to_string(), format!("base={expected}"));
        let rtc = format!("base={input},clock=vm,driftfix=slew").parse::<Rtc>().unwrap();
        let args = rtc.to_args();
        assert_eq!(args, [format!("base={expected},clock=vm,driftfix=slew")]);
        assert_eq!(args[0].parse::<Rtc>().unwrap(), rtc);
    }
}

#[test]
fn rtc_rejects_invalid_dates_and_unsupported_suffixes() {
    for input in [
        "2023-02-29",
        "1900-02-29",
        "2024-13-01",
        "2024-01-00",
        "2024-01-01T24:00:00",
        "2024-01-01T00:60:00",
        "2024-01-01T00:00",
        "2024-01-01T00:00:00Z",
        "2024-01-01T00:00:00+01:00",
        "2024-01-01T00:00:00.123",
        "2024-01-01junk",
        "",
    ] {
        assert!(input.parse::<RtcBase>().is_err(), "accepted {input:?}");
        assert!(format!("base={input}").parse::<Rtc>().is_err(), "accepted {input:?}");
    }
}
