use pretty_assertions::assert_eq;
use qemu_command_builder::args::device::Device;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn device_displays_driver_and_properties() {
    let mut device = Device::new("virtio-net-pci");
    device.add_prop("mac", "f2:3c:93:6e:bb:d4").add_prop("netdev", "net0").add_prop("vectors", "18").add_prop("mq", "on");

    assert_eq!("virtio-net-pci,mac=f2:3c:93:6e:bb:d4,mq=on,netdev=net0,vectors=18", device.to_args()[0]);
}

#[test]
fn device_parses_flag_properties() {
    let parsed = Device::from_str("driver-with-help,help").unwrap();

    assert_eq!("driver-with-help,help", parsed.to_args()[0]);
    assert_eq!(parsed, Device::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn device_round_trips_punctuated_property_values() {
    let parsed = Device::from_str("virtio-9p-pci,fsdev=fs0,mount_tag=host/share,addr=0x5").unwrap();

    assert_eq!("virtio-9p-pci,addr=0x5,fsdev=fs0,mount_tag=host/share", parsed.to_args()[0]);
    assert_eq!(parsed, Device::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn device_parses_scsi_hd_properties() {
    let parsed = Device::from_str("scsi-hd,bus=scsi0.0,scsi-id=0,channel=0,lun=0,drive=drive-scsi-disk-0,id=drive0,bootindex=1,rotation_rate=1").unwrap();

    assert_eq!(
        "scsi-hd,bootindex=1,bus=scsi0.0,channel=0,drive=drive-scsi-disk-0,id=drive0,lun=0,rotation_rate=1,scsi-id=0",
        parsed.to_args()[0]
    );
    assert_eq!(parsed, Device::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn device_preserves_json_and_documents_smmuv3_auto_properties() {
    let json = r#"{"driver":"virtio-net-pci","id":"net0","vectors":18}"#;
    assert_eq!(json, Device::from_str(json).unwrap().to_args()[0]);

    let mut smmu = Device::arm_smmuv3();
    smmu.add_prop("ril", "auto")
        .add_prop("ats", "auto")
        .add_prop("oas", "auto")
        .add_prop("ssidsize", "auto")
        .add_prop("cmdqv", "auto");
    assert_eq!("arm-smmuv3,ats=auto,cmdqv=auto,oas=auto,ril=auto,ssidsize=auto", smmu.to_args()[0]);
}
