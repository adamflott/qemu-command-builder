use pretty_assertions::assert_eq;
use qemu_command_builder::args::acpitable::{AcpiTable, AcpiTableData};
use qemu_command_builder::shell_path::ShellPath;
use qemu_command_builder::shell_string::ShellString;
use qemu_command_builder::to_command::ToCommand;
use std::str::FromStr;

#[test]
fn acpitable_displays_file_payloads() {
    let table = AcpiTable::builder()
        .sig(ShellString::from("SLIC"))
        .oem_id(ShellString::from("ACME"))
        .data(AcpiTableData::File(vec![ShellPath::from("/tmp/slic.bin"), ShellPath::from("/tmp/slic2.bin")]))
        .build();

    assert_eq!("sig=SLIC,oem_id=ACME,file=/tmp/slic.bin:/tmp/slic2.bin", table.to_args()[0]);
}

#[test]
fn acpitable_round_trips_data_payloads() {
    let s = "sig=DSDT,rev=2,oem_id=ACME,oem_table_id=TABLE1,oem_rev=7,asl_compiler_id=INTL,asl_compiler_rev=202403,data=/tmp/dsdt.aml:/tmp/dsdt-2.aml";

    let parsed = AcpiTable::from_str(s).unwrap();

    assert_eq!(s, parsed.to_args()[0]);
    assert_eq!(parsed, AcpiTable::from_str(&parsed.to_args()[0]).unwrap());
}

#[test]
fn acpitable_parses_mixed_order_into_canonical_output() {
    let parsed = AcpiTable::from_str("file=/tmp/slic.bin:/tmp/slic2.bin,oem_table_id='OEM TABLE',sig=SLIC").unwrap();

    assert_eq!("sig=SLIC,oem_table_id=OEM TABLE,file=/tmp/slic.bin:/tmp/slic2.bin", parsed.to_args()[0]);
    assert_eq!(parsed, AcpiTable::from_str(&parsed.to_args()[0]).unwrap());
}
