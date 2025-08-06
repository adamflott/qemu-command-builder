use std::path::PathBuf;

use bon::Builder;

use crate::to_command::ToCommand;

pub enum DataFile {
    File(Vec<PathBuf>),
    Data(Vec<PathBuf>),
}

/// Add ACPI table with specified header fields and context from
/// specified files. For file=, take whole ACPI table from the specified
/// files, including all ACPI headers (possible overridden by other
/// options). For data=, only data portion of the table is used, all
/// header information is specified in the command line. If a SLIC table
/// is supplied to QEMU, then the SLIC's oem\_id and oem\_table\_id
/// fields will override the same in the RSDT and the FADT (a.k.a.
/// FACP), in order to ensure the field matches required by the
/// Microsoft SLIC spec and the ACPI spec.
#[derive(Default, Builder)]
pub struct AcpiTable {
    sig: Option<String>,
    rev: Option<usize>,
    oem_id: Option<String>,
    oem_table_id: Option<String>,
    oem_rev: Option<usize>,
    asl_compiler_id: Option<String>,
    asl_compiler_rev: Option<usize>,
    data: Option<DataFile>,
}

impl ToCommand for AcpiTable {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-acpitable".to_string());

        let mut args = vec![];
        if let Some(sig) = &self.sig {
            args.push(format!("sig={}", sig));
        }
        if let Some(rev) = self.rev {
            args.push(format!("rev={}", rev));
        }
        if let Some(oem_id) = &self.oem_id {
            args.push(format!("oem_id={}", oem_id));
        }
        if let Some(oem_table_id) = &self.oem_table_id {
            args.push(format!("oem_table_id={}", oem_table_id));
        }
        if let Some(oem_rev) = self.oem_rev {
            args.push(format!("oem_rev={}", oem_rev));
        }
        if let Some(asl_compiler_id) = &self.asl_compiler_id {
            args.push(format!("asl_compiler_id={}", asl_compiler_id));
        }
        if let Some(asl_compiler_rev) = self.asl_compiler_rev {
            args.push(format!("asl_compiler_rev={}", asl_compiler_rev));
        }
        if let Some(data) = &self.data {
            match data {
                DataFile::File(data) => {
                    let files: Vec<String> = data.iter().map(|p| p.display().to_string()).collect();
                    args.push(format!("file={}", files.join(":")));
                }
                DataFile::Data(data) => {
                    let files: Vec<String> = data.iter().map(|p| p.display().to_string()).collect();
                    args.push(format!("file={}", files.join(":")));
                }
            }
        }

        cmd.push(args.join(","));
        cmd
    }
}
