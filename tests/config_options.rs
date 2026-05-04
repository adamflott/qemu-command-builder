use pretty_assertions::assert_eq;
use qemu_command_builder::common::OnOff;
use qemu_command_builder::fw_cfg::FwCfg;
use qemu_command_builder::global::Global;
use qemu_command_builder::icount::{Icount, RecordReplay, Shift};
use qemu_command_builder::to_command::ToCommand;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn global_accepts_both_qemu_forms() {
    let shorthand = Global::from_str("ide-hd.physical_block_size=4096").unwrap();
    let longhand = Global::from_str("driver=ide-hd,property=physical_block_size,value=4096").unwrap();

    assert_eq!("driver=ide-hd,property=physical_block_size,value=4096", shorthand.to_args()[0]);
    assert_eq!(shorthand, longhand);
}

#[test]
fn fw_cfg_accepts_name_alias_and_round_trips() {
    let file_cfg = FwCfg::from_str("opt/com.mycompany/blob,file=./my_blob.bin").unwrap();
    let string_cfg = FwCfg::from_str("name=opt/com.mycompany/cmdline,string=console=ttyS0").unwrap();

    assert_eq!("name=opt/com.mycompany/blob,file=./my_blob.bin", file_cfg.to_args()[0]);
    assert_eq!(file_cfg, FwCfg::from_str(&file_cfg.to_args()[0]).unwrap());
    assert_eq!("name=opt/com.mycompany/cmdline,string=console=ttyS0", string_cfg.to_args()[0]);
    assert_eq!(string_cfg, FwCfg::from_str(&string_cfg.to_args()[0]).unwrap());
}

#[test]
fn icount_round_trips_record_replay_options() {
    let icount = Icount::builder()
        .shift(Shift::Auto)
        .align(OnOff::Off)
        .sleep(OnOff::On)
        .rr(RecordReplay::Record)
        .rrfile(PathBuf::from("/tmp/replay.bin"))
        .rrsnapshot("snap0".to_string())
        .build();

    let rendered = icount.to_args()[0].clone();
    assert_eq!("shift=auto,align=off,sleep=on,rr=record,rrfile=/tmp/replay.bin,rrsnapshot=snap0", rendered);
    assert_eq!(icount, Icount::from_str(&rendered).unwrap());
}
