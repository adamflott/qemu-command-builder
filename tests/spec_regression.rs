use pretty_assertions::assert_eq;
use qemu_command_builder::args::accel::Accel;
use qemu_command_builder::args::acpitable::AcpiTable;
use qemu_command_builder::args::action::Action;
use qemu_command_builder::args::addfs::AddFd;
use qemu_command_builder::args::audiodev::AudioDev;
use qemu_command_builder::args::blockdev::BlockDev;
use qemu_command_builder::args::boot::Boot;
use qemu_command_builder::args::chardev::CharDev;
use qemu_command_builder::args::device::Device;
use qemu_command_builder::args::display::QemuDisplay;
use qemu_command_builder::args::drive::Drive;
use qemu_command_builder::args::fsdev::FsDev;
use qemu_command_builder::args::fw_cfg::FwCfg;
use qemu_command_builder::args::machine::MachineX86_64;
use qemu_command_builder::args::memory::Memory;
use qemu_command_builder::args::mon::Mon;
use qemu_command_builder::args::netdev::NetDev;
use qemu_command_builder::args::numa::NUMA;
use qemu_command_builder::args::object::Object;
use qemu_command_builder::args::plugin::Plugin;
use qemu_command_builder::args::rtc::Rtc;
use qemu_command_builder::args::sandbox::Sandbox;
use qemu_command_builder::args::semihosting::SemihostingConfig;
use qemu_command_builder::args::smbios::Smbios;
use qemu_command_builder::args::smp::SMP;
use qemu_command_builder::args::tpmdev::TpmDev;
use qemu_command_builder::args::trace::Trace;
use qemu_command_builder::args::virtfs::Virtfs;
use qemu_command_builder::to_command::ToCommand;
use std::fmt::Debug;
use std::str::FromStr;

fn assert_one_value_argv<T>(input: &str, option: &str)
where
    T: ToCommand + FromStr + Debug,
    T::Err: Debug,
{
    let value = input.parse::<T>().unwrap();
    let argv = value.to_command();
    assert_eq!(2, argv.len(), "{option} split its value across argv entries: {argv:?}");
    assert_eq!(option, argv[0]);
    assert_eq!(value, argv[1].parse::<T>().unwrap(), "{option} did not round-trip its single argv value");
}

#[test]
fn comma_valued_options_remain_single_argv_entries() {
    assert_one_value_argv::<MachineX86_64>("q35,accel=kvm,nvdimm=on", "-machine");
    assert_one_value_argv::<Accel>("tcg,thread=multi,split-wx=on", "-accel");
    assert_one_value_argv::<SMP>("4,sockets=2,cores=2", "-smp");
    assert_one_value_argv::<NUMA>("node,nodeid=0,mem=1024", "-numa");
    assert_one_value_argv::<AddFd>("fd=3,set=1,opaque=disk", "-add-fd");
    assert_one_value_argv::<Boot>("order=cdn,menu=on,strict=off", "-boot");
    assert_one_value_argv::<Memory>("1G,slots=2,maxmem=4G", "-m");
    assert_one_value_argv::<AudioDev>("none,id=audio0,timer-period=10000", "-audiodev");
    assert_one_value_argv::<Device>("virtio-net-pci,id=net0,netdev=n0", "-device");
    assert_one_value_argv::<BlockDev>("driver=raw,node-name=disk,file=disk-file", "-blockdev");
    assert_one_value_argv::<Drive>("file=/tmp/disk.img,if=none,format=raw", "-drive");
    assert_one_value_argv::<FsDev>("synth,id=fs0,max_xattr=32", "-fsdev");
    assert_one_value_argv::<Virtfs>("synth,mount_tag=tag0,max_xattr=32", "-virtfs");
    assert_one_value_argv::<QemuDisplay>("gtk,clipboard=on,full-screen=off", "-display");
    assert_one_value_argv::<AcpiTable>("sig=DSDT,file=/tmp/dsdt.aml", "-acpitable");
    assert_one_value_argv::<Smbios>("type=1,manufacturer=Acme,product=VM", "-smbios");
    assert_one_value_argv::<NetDev>("user,id=net0,ipv6=off", "-netdev");
    assert_one_value_argv::<CharDev>("stdio,id=char0,mux=off", "-chardev");
    assert_one_value_argv::<TpmDev>("emulator,id=tpm0,chardev=char0", "-tpmdev");
    assert_one_value_argv::<FwCfg>("name=opt/test,string=value", "-fw_cfg");
    assert_one_value_argv::<Mon>("chardev=char0,mode=control,pretty=on", "-mon");
    assert_one_value_argv::<Action>("reboot=shutdown,shutdown=pause,panic=none", "-action");
    assert_one_value_argv::<Rtc>("base=utc,clock=vm,driftfix=slew", "-rtc");
    assert_one_value_argv::<Sandbox>("on,obsolete=deny,spawn=allow", "-sandbox");
    assert_one_value_argv::<Trace>("enable=net_*,events=/tmp/events,file=/tmp/trace", "-trace");
    assert_one_value_argv::<Plugin>("file=/tmp/plugin.so,arg=value", "-plugin");
    assert_one_value_argv::<SemihostingConfig>("enable=on,target=native,arg=one", "-semihosting-config");
    assert_one_value_argv::<Object>("iothread,id=io0,poll-weight=3", "-object");
}

#[test]
fn every_qemu_11_1_top_level_option_is_declared_in_the_parser_catalog() {
    let spec = include_str!("../spec/qemu-options-11.1.0.hx");
    let parser_catalog = include_str!("../src/parsers.rs");
    let mut missing = Vec::new();

    for line in spec.lines() {
        let Some(rest) = line.strip_prefix("DEF(\"") else { continue };
        let Some((name, _)) = rest.split_once('"') else { continue };
        if matches!(name, "help" | "version") {
            continue;
        }
        let spelling = match name {
            "M" => "-M".to_string(),
            "preconfig" => "--preconfig".to_string(),
            _ => format!("-{name}"),
        };
        if !parser_catalog.contains(&format!("\"{spelling}\"")) {
            missing.push(name.to_string());
        }
    }

    assert!(missing.is_empty(), "QEMU options missing from src/parsers.rs: {missing:?}");
}

#[test]
fn required_identifiers_and_mutually_constrained_values_are_rejected() {
    for invalid in ["none", "alsa,out.frequency=48000"] {
        assert!(AudioDev::from_str(invalid).is_err(), "accepted invalid -audiodev {invalid}");
    }
    assert!(FsDev::from_str("synth").is_err());
    assert!(FsDev::from_str("local,id=fs0,path=/tmp,security_model=invalid").is_err());
    assert!(CharDev::from_str("stdio,mux=on").is_err());
    assert!(NetDev::from_str("tap").is_err());
    assert!(TpmDev::from_str("emulator,id=tpm0").is_err());
    assert!(SemihostingConfig::from_str("target=invalid").is_err());
    assert!(Memory::from_str("1G,slots=2").is_err());
    assert!(Memory::from_str("1G,maxmem=4G").is_err());
}

#[test]
fn key_value_parsers_accept_arbitrary_property_order() {
    macro_rules! same {
        ($type:ty, $left:expr, $right:expr) => {
            assert_eq!(<$type>::from_str($left).unwrap(), <$type>::from_str($right).unwrap());
        };
    }
    same!(Accel, "tcg,thread=multi,split-wx=on", "tcg,split-wx=on,thread=multi");
    same!(SMP, "4,sockets=2,cores=2", "cores=2,cpus=4,sockets=2");
    same!(Boot, "order=cdn,menu=on,strict=off", "strict=off,order=cdn,menu=on");
    same!(Memory, "1G,slots=2,maxmem=4G", "size=1G,maxmem=4G,slots=2");
    same!(FsDev, "synth,id=fs0,max_xattr=32", "synth,max_xattr=32,id=fs0");
    same!(Virtfs, "synth,mount_tag=tag0,max_xattr=32", "synth,max_xattr=32,mount_tag=tag0");
    same!(CharDev, "stdio,id=char0,mux=off", "stdio,mux=off,id=char0");
    same!(TpmDev, "emulator,id=tpm0,chardev=char0", "emulator,chardev=char0,id=tpm0");
    same!(Rtc, "base=utc,clock=vm,driftfix=slew", "driftfix=slew,base=utc,clock=vm");
    same!(Sandbox, "on,obsolete=deny,spawn=allow", "on,spawn=allow,obsolete=deny");
}
