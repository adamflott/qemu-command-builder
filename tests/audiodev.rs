use pretty_assertions::assert_eq;
use qemu_command_builder::QemuInstanceForX86_64;
use qemu_command_builder::args::audiodev::AudioDev;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn audiodev_displays_canonical_driver_and_properties() {
    let audiodev = AudioDev::builder()
        .driver("alsa".to_string())
        .props(vec![
            qemu_command_builder::args::audiodev::AudioDevProperty {
                key: "out.frequency".to_string(),
                value: Some("8000".to_string()),
            },
            qemu_command_builder::args::audiodev::AudioDevProperty {
                key: "id".to_string(),
                value: Some("example".to_string()),
            },
            qemu_command_builder::args::audiodev::AudioDevProperty {
                key: "out.channels".to_string(),
                value: Some("1".to_string()),
            },
        ])
        .build();

    assert_eq!("alsa,out.frequency=8000,id=example,out.channels=1", audiodev.to_args()[0]);
}

#[test]
fn audiodev_accepts_driver_prefix_and_flag_properties() {
    let parsed = AudioDev::from_str("driver=pa,id=pa0,server=/run/pulse.sock,try-poll").unwrap();

    assert_eq!("pa,id=pa0,server=/run/pulse.sock,try-poll", parsed.to_args()[0]);
    assert_eq!(parsed, AudioDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn audiodev_preserves_repeated_property_order() {
    let parsed = AudioDev::from_str("wav,id=wav0,out.frequency=48000,out.frequency=44100").unwrap();

    assert_eq!("wav,id=wav0,out.frequency=48000,out.frequency=44100", parsed.to_args()[0]);
}

#[test]
fn qemu_instance_parses_audiodev() {
    let cmd = "/usr/bin/qemu-system-x86_64 -audiodev none,id=audio0 -nodefaults";
    let parsed = QemuInstanceForX86_64::from_str(cmd).unwrap();

    assert_eq!(cmd, parsed.to_single_command());
}
