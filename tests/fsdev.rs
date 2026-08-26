use pretty_assertions::assert_eq;
use qemu_command_builder::args::fsdev::{FsDev, FsDevLocal, FsDevSynth, SecurityModel};
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn fsdev_local_displays_single_canonical_argument() {
    let fsdev = FsDev::Local(Box::new(
        FsDevLocal::builder()
            .id("fs0".to_string())
            .path(PathBuf::from("/exports/share"))
            .security_model(SecurityModel::MappedXAttr)
            .writeout(())
            .readonly(())
            .build(),
    ));

    assert_eq!("local,id=fs0,path=/exports/share,security_model=mapped-xattr,writeout=immediate,readonly=on", fsdev.to_args()[0]);
}

#[test]
fn fsdev_local_round_trips_throttling_options() {
    let parsed =
        FsDev::from_str("local,id=fs0,path=/exports/share,security_model=passthrough,throttling.bps-total=1024,throttling.bps-read-max=2048,throttling.iops-total-max=99,throttling.iops-size=4096")
            .unwrap();

    assert_eq!(
        "local,id=fs0,path=/exports/share,security_model=passthrough,throttling.bps-total=1024,throttling.bps-read-max=2048,throttling.iops-total-max=99,throttling.iops-size=4096",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, FsDev::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn fsdev_synth_round_trips_readonly() {
    let fsdev = FsDev::Synth(FsDevSynth::builder().id("s0".to_string()).readonly(()).build());

    let rendered = fsdev.to_args()[0].clone();

    assert_eq!("synth,id=s0,readonly=on", rendered);
    assert_eq!(fsdev, FsDev::from_str(&rendered).unwrap());
}

#[test]
fn fsdev_round_trips_qemu_11_1_max_xattr() {
    let local = FsDev::from_str("local,id=fs0,path=/exports/share,security_model=none,max_xattr=256").unwrap();
    let synth = FsDev::from_str("synth,id=s0,max_xattr=0").unwrap();

    assert_eq!("local,id=fs0,path=/exports/share,security_model=none,max_xattr=256", local.to_args()[0]);
    assert_eq!("synth,id=s0,max_xattr=0", synth.to_args()[0]);
}

#[test]
fn fsdev_uses_fixed_width_u64_quantities() {
    let value = "local,id=fs0,path=/exports/share,security_model=none,throttling.bps-total=5000000000";
    assert_eq!(value, FsDev::from_str(value).unwrap().to_args()[0]);
}
