use pretty_assertions::assert_eq;
use qemu_command_builder::args::audio::Audio;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn audio_displays_driver_model_and_properties() {
    let mut audio = Audio::new("pa");
    audio.model("sb16");
    audio.add_prop("id", "pa0");
    audio.add_prop("out.frequency", "44100");

    assert_eq!("driver=pa,model=sb16,id=pa0,out.frequency=44100", audio.to_args()[0]);
}

#[test]
fn audio_parses_bare_driver_and_flag_properties() {
    let parsed = Audio::from_str("none,try-poll").unwrap();

    assert_eq!("driver=none,try-poll", parsed.to_args()[0]);
    assert_eq!(parsed, Audio::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn audio_parses_driver_equals_and_round_trips() {
    let parsed = Audio::from_str("driver=pa,model=sb16,out.channels=1,in.fixed-settings").unwrap();

    assert_eq!("driver=pa,model=sb16,out.channels=1,in.fixed-settings", parsed.to_args()[0]);
    assert_eq!(parsed, Audio::from_str(&parsed.to_args()[0]).unwrap());
}
