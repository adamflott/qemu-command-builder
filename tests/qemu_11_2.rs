use pretty_assertions::assert_eq;
use qemu_command_builder::args::compact::{AcceptHide, AcceptRejectCrash, Compact, CompatPolicies};
use qemu_command_builder::args::cpu::{CpuHexagon, CpuS390x};
use qemu_command_builder::args::machine_hexagon::MachineHexagon;
use qemu_command_builder::args::machine_s390x::MachineS390x;
use qemu_command_builder::args::netdev::{NetDev, ScriptOrNot, Tap};
use qemu_command_builder::args::object::Object;
use qemu_command_builder::args::semihosting::{SemihostingConfig, SemihostingTarget};
use qemu_command_builder::common::{AccelType, OnOff, OnOffDefaultOff};
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use qemu_command_builder::{QemuCommand, QemuInstanceForHexagon, QemuInstanceForS390x, QemuInstanceForX86_64};
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn compat_accepts_each_input_policy_independently() {
    for key in ["deprecated-input", "unstable-input", "insecure-input"] {
        for policy in ["accept", "warn", "reject", "crash"] {
            let value = format!("{key}={policy}");
            let parsed = Compact::from_str(&value).unwrap();
            assert_eq!(vec!["-compat", &value], parsed.to_command());
            assert_eq!(parsed, Compact::from_str(&parsed.to_args()[0]).unwrap());
        }
    }
    for key in ["deprecated-output", "unstable-output"] {
        for policy in ["accept", "hide"] {
            let value = format!("{key}={policy}");
            assert_eq!(vec![value.clone()], Compact::from_str(&value).unwrap().to_args());
        }
    }
}

#[test]
fn compat_builds_and_parses_combined_policies() {
    let policies = Compact::Policies(
        CompatPolicies::builder()
            .deprecated_input(AcceptRejectCrash::Warn)
            .deprecated_output(AcceptHide::Hide)
            .unstable_input(AcceptRejectCrash::Warn)
            .unstable_output(AcceptHide::Accept)
            .insecure_input(AcceptRejectCrash::Reject)
            .build(),
    );
    let value = "deprecated-input=warn,deprecated-output=hide,unstable-input=warn,unstable-output=accept,insecure-input=reject";
    assert_eq!(vec!["-compat", value], policies.to_command());
    assert_eq!(policies, Compact::from_str(value).unwrap());
    let reordered = "insecure-input=reject,unstable-output=accept,deprecated-output=hide,unstable-input=warn,deprecated-input=warn";
    assert_eq!(policies, Compact::from_str(reordered).unwrap());

    let cmd = format!("qemu-system-x86_64 -compat {value}");
    assert_eq!(cmd, QemuInstanceForX86_64::from_str(&cmd).unwrap().to_single_command());
}

#[test]
fn compat_rejects_invalid_policy_values() {
    for value in [
        "",
        "insecure-input=hide",
        "insecure-input=",
        "insecure-output=hide",
        "deprecated-output=warn",
        "unstable-output=warn",
        "deprecated-input=invalid",
    ] {
        assert!(Compact::from_str(value).is_err(), "accepted {value:?}");
    }
}

#[test]
fn tap_empty_script_values_disable_execution_and_legacy_no_is_normalized() {
    let value = NetDev::Tap(Tap::builder().id("net0".to_string()).script(ScriptOrNot::None).downscript(ScriptOrNot::None).build());
    let canonical = "tap,id=net0,script=,downscript=";
    assert_eq!(vec!["-netdev", canonical], value.to_command());
    assert_eq!(value, NetDev::from_str(canonical).unwrap());
    assert_eq!(value, NetDev::from_str("tap,id=net0,script=no,downscript=no").unwrap());
    assert_eq!(ScriptOrNot::None, ScriptOrNot::from_str("").unwrap());
    assert_eq!(ScriptOrNot::None, ScriptOrNot::from_str("no").unwrap());

    // Omission retains QEMU's defaults; a real script path remains a path.
    for input in ["tap,id=net0", "tap,id=net0,script=/host/ifup,downscript=/host/ifdown", "tap,id=net0,script=./no"] {
        assert_eq!(vec![input], NetDev::from_str(input).unwrap().to_args());
    }
    assert_eq!(ScriptOrNot::Script(PathBuf::from("./no")), ScriptOrNot::from_str("./no").unwrap());
}

#[test]
fn s390x_boot_certificates_and_secure_boot_build_and_parse() {
    let machine = MachineS390x::builder()
        .machine_type("s390-ccw-virtio-11.2")
        .accel(vec![AccelType::Kvm, AccelType::Tcg])
        .boot_certs(vec![ShellString::new("/host/certs"), ShellString::new("/host/signing cert.pem")])
        .secure_boot(OnOffDefaultOff::On)
        .extra_properties(vec![(ShellString::new("aes-key-wrap"), ShellString::new("off"))])
        .build();
    let value = "s390-ccw-virtio-11.2,accel=kvm:tcg,boot-certs.0.path=/host/certs,boot-certs.1.path=/host/signing cert.pem,secure-boot=on,aes-key-wrap=off";
    assert_eq!(vec!["-machine", value], machine.to_command());
    assert_eq!(machine, MachineS390x::from_str(value).unwrap());
    let reordered = "secure-boot=on,boot-certs.1.path=/host/signing cert.pem,type=s390-ccw-virtio-11.2,accel=kvm:tcg,boot-certs.0.path=/host/certs,aes-key-wrap=off";
    assert_eq!(machine, MachineS390x::from_str(reordered).unwrap());

    assert_eq!(vec!["s390-ccw-virtio"], MachineS390x::builder().machine_type("s390-ccw-virtio").build().to_args());
    let disabled = "s390-ccw-virtio,secure-boot=off";
    assert_eq!(vec![disabled], MachineS390x::from_str(disabled).unwrap().to_args());
}

#[test]
fn s390x_rejects_malformed_secure_boot_and_certificate_properties() {
    for value in [
        "",
        "type=",
        "secure-boot=on",
        "s390-ccw-virtio,secure-boot=invalid",
        "s390-ccw-virtio,boot-certs.0=/cert.pem",
        "s390-ccw-virtio,boot-certs.x.path=/cert.pem",
        "s390-ccw-virtio,boot-certs.0.other=/cert.pem",
        "s390-ccw-virtio,boot-certs.1.path=/cert.pem",
        "s390-ccw-virtio,boot-certs.0.path=/a.pem,boot-certs.2.path=/b.pem",
    ] {
        assert!(MachineS390x::from_str(value).is_err(), "accepted {value:?}");
    }
}

#[test]
fn s390x_complete_commands_preserve_argv_and_shell_paths() {
    let machine = MachineS390x::builder()
        .machine_type("s390-ccw-virtio")
        .boot_certs(vec![ShellString::new("/host/signing cert.pem")])
        .secure_boot(OnOffDefaultOff::On)
        .build();
    let instance = QemuInstanceForS390x::builder()
        .qemu_binary(PathBuf::from("/usr/bin/qemu-system-s390x"))
        .machine(machine)
        .cpu(CpuS390x::builder().cpu_type("max").properties(vec![("unpack".to_string(), "on".to_string())]).build())
        .compact(Compact::Policies(CompatPolicies::builder().insecure_input(AcceptRejectCrash::Warn).build()))
        .build();
    let argv = instance.to_command();
    assert_eq!(
        vec![
            "/usr/bin/qemu-system-s390x",
            "-machine",
            "s390-ccw-virtio,boot-certs.0.path=/host/signing cert.pem,secure-boot=on",
            "-cpu",
            "max,unpack=on",
            "-compat",
            "insecure-input=warn",
        ],
        argv
    );
    let shell = instance.to_single_command();
    assert_eq!(instance, QemuInstanceForS390x::from_str(&shell).unwrap());
    let parsed = QemuCommand::from_str(&shell).unwrap();
    assert!(matches!(parsed, QemuCommand::S390x(_)));
    assert_eq!(argv, parsed.to_command());
    assert_eq!(vec!["max,unpack=on,msa=off"], CpuS390x::from_str("max,+unpack,-msa").unwrap().to_args());
    let alias = QemuInstanceForS390x::from_str("qemu-system-s390x -M s390-ccw-virtio,secure-boot=off").unwrap();
    assert_eq!("qemu-system-s390x -machine s390-ccw-virtio,secure-boot=off", alias.to_single_command());
}

#[test]
fn igvm_cmdline_is_preserved_including_spaces_equals_and_literal_commas() {
    for cmdline in ["", "console=ttyS0 root=/dev/vda", "console=ttyS0,115200 root=/dev/vda", "args=one,,two, key=value=other 'quoted' 雪"] {
        let mut igvm = Object::igvm_cfg("igvm0", "/firmware/guest image.igvm");
        igvm.add_prop("cmdline", cmdline);
        let expected = format!("igvm-cfg,id=igvm0,file=/firmware/guest image.igvm,cmdline={}", cmdline.replace(',', ",,"));
        assert_eq!(vec!["-object", &expected], igvm.to_command());
        assert_eq!(igvm, Object::from_str(&expected).unwrap());
        let instance = QemuInstanceForX86_64::builder().qemu_binary(PathBuf::from("qemu-system-x86_64")).object(vec![igvm]).build();
        assert_eq!(instance.to_command(), QemuInstanceForX86_64::from_str(&instance.to_single_command()).unwrap().to_command());
    }
    let no_cmdline = Object::igvm_cfg("igvm0", "/firmware/guest.igvm");
    assert_eq!(vec!["igvm-cfg,id=igvm0,file=/firmware/guest.igvm"], no_cmdline.to_args());
}

#[test]
fn hexagon_semihosting_builds_and_parses_complete_commands() {
    let instance = QemuInstanceForHexagon::builder()
        .qemu_binary(PathBuf::from("/usr/bin/qemu-system-hexagon"))
        .machine(MachineHexagon::builder().machine_type("V68N_1024").build())
        .cpu(CpuHexagon::builder().cpu_type("v68").build())
        .semihosting(true)
        .semihosting_config(
            SemihostingConfig::builder()
                .enable(OnOff::On)
                .target(SemihostingTarget::Native)
                .userspace(OnOff::Off)
                .args(vec!["one".to_string(), "two".to_string()])
                .build(),
        )
        .build();
    let command = "/usr/bin/qemu-system-hexagon -machine V68N_1024 -cpu v68 -semihosting -semihosting-config enable=on,target=native,userspace=off,arg=one,arg=two";
    assert_eq!(command, instance.to_single_command());
    assert_eq!(instance, QemuInstanceForHexagon::from_str(command).unwrap());
    let parsed = QemuCommand::from_str(command).unwrap();
    assert!(matches!(parsed, QemuCommand::Hexagon(_)));
    assert_eq!(instance.to_command(), parsed.to_command());

    let disabled = "qemu-system-hexagon -semihosting-config enable=off";
    assert_eq!(disabled, QemuInstanceForHexagon::from_str(disabled).unwrap().to_single_command());
    let alias = QemuInstanceForHexagon::from_str("qemu-system-hexagon -M type=virt").unwrap();
    assert_eq!("qemu-system-hexagon -machine virt", alias.to_single_command());
}

#[test]
fn hexagon_machine_properties_and_unenumerated_cpu_models_are_preserved() {
    let machine = MachineHexagon::builder()
        .machine_type("virt")
        .properties(vec![(ShellString::new("memory-backend"), ShellString::new("ram0"))])
        .build();
    assert_eq!(vec!["virt,memory-backend=ram0"], machine.to_args());
    assert_eq!(machine, MachineHexagon::from_str("memory-backend=ram0,type=virt").unwrap());
    for invalid in ["", "type=", "memory-backend=ram0", "virt,invalid"] {
        assert!(MachineHexagon::from_str(invalid).is_err());
    }
    for invalid in ["", "max,+", "max,-", "max,=on", "max,invalid"] {
        assert!(CpuS390x::from_str(invalid).is_err());
        assert!(CpuHexagon::from_str(invalid).is_err());
    }
}
