use pretty_assertions::assert_eq;
use qemu_command_builder::args::network_compat::{LegacyNet, Nic};
use qemu_command_builder::args::semihosting::SemihostingConfig;
use qemu_command_builder::to_command::ToCommand;
use qemu_command_builder::{QemuInstanceForAarch64, QemuInstanceForX86_64};
use std::str::FromStr;

#[test]
fn machine_m_alias_parses_to_canonical_machine_option() {
    let parsed = QemuInstanceForX86_64::from_str("/usr/bin/qemu-system-x86_64 -M q35").unwrap();
    assert_eq!("/usr/bin/qemu-system-x86_64 -machine q35", parsed.to_single_command());
}

#[test]
fn nic_and_legacy_net_preserve_properties() {
    let nic = Nic::from_str("user,ipv6=off,model=e1000,mac=52:54:98:76:54:32").unwrap();
    let none = Nic::from_str("none").unwrap();
    let net = LegacyNet::from_str("nic,macaddr=52:54:00:12:34:56,model=virtio").unwrap();

    assert_eq!(vec!["user,ipv6=off,model=e1000,mac=52:54:98:76:54:32"], nic.to_args());
    assert_eq!(vec!["none"], none.to_args());
    assert_eq!(vec!["nic,macaddr=52:54:00:12:34:56,model=virtio"], net.to_args());
}

#[test]
fn semihosting_config_preserves_repeated_args() {
    let config = SemihostingConfig::from_str("enable=on,target=native,chardev=console0,userspace=off,arg=one,arg=two").unwrap();
    assert_eq!(vec!["enable=on,target=native,chardev=console0,userspace=off,arg=one,arg=two"], config.to_args());

    let cmd = "/usr/bin/qemu-system-aarch64 -semihosting -semihosting-config enable=on,target=native,arg=one,arg=two";
    assert_eq!(cmd, QemuInstanceForAarch64::from_str(cmd).unwrap().to_single_command());
}

#[test]
fn internal_qtest_and_prom_env_options_round_trip() {
    let cmd = "/usr/bin/qemu-system-x86_64 -qtest unix:/tmp/qtest.sock -qtest-log /tmp/qtest.log -prom-env auto-boot?=false -prom-env boot-device=hd:2";
    assert_eq!(
        vec![
            "/usr/bin/qemu-system-x86_64",
            "-qtest",
            "unix:/tmp/qtest.sock",
            "-qtest-log",
            "/tmp/qtest.log",
            "-prom-env",
            "auto-boot?=false",
            "-prom-env",
            "boot-device=hd:2",
        ],
        QemuInstanceForX86_64::from_str(cmd).unwrap().to_command(),
    );
}

#[test]
fn qemu_instance_round_trips_nic_and_legacy_net() {
    let cmd = "/usr/bin/qemu-system-x86_64 -nic none -net nic,model=e1000";
    assert_eq!(cmd, QemuInstanceForX86_64::from_str(cmd).unwrap().to_single_command());
}
