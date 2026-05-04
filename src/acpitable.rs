use crate::parsers::ARG_ACPITABLE;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::parsers::{DELIM_COLON, DELIM_COMMA};
use crate::qao;
use crate::shell_path::ShellPath;
use crate::shell_string::ShellString;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;

const KEY_SIG: &str = "sig=";
const KEY_REV: &str = "rev=";
const KEY_OEM_ID: &str = "oem_id=";
const KEY_OEM_TABLE_ID: &str = "oem_table_id=";
const KEY_OEM_REV: &str = "oem_rev=";
const KEY_ASL_COMPILER_ID: &str = "asl_compiler_id=";
const KEY_ASL_COMPILER_REV: &str = "asl_compiler_rev=";
const KEY_FILE: &str = "file=";
const KEY_DATA: &str = "data=";

/// ACPI table payload source for `-acpitable`.
///
/// `file=` passes one or more complete ACPI table files including headers.
/// `data=` passes one or more payload files while the table header fields are
/// supplied on the command line.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum AcpiTableData {
    /// Emit `file=file1[:file2]...`.
    File(Vec<ShellPath>),
    /// Emit `data=file1[:file2]...`.
    Data(Vec<ShellPath>),
}

impl AcpiTableData {
    fn key(&self) -> &'static str {
        match self {
            AcpiTableData::File(_) => KEY_FILE,
            AcpiTableData::Data(_) => KEY_DATA,
        }
    }

    fn files(&self) -> &[ShellPath] {
        match self {
            AcpiTableData::File(files) | AcpiTableData::Data(files) => files,
        }
    }
}

impl Display for AcpiTableData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let files = self.files().iter().map(|p| p.as_ref()).collect::<Vec<_>>().join(DELIM_COLON);
        write!(f, "{}{}", self.key(), files)
    }
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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct AcpiTable {
    /// ACPI signature override.
    sig: Option<ShellString>,
    /// ACPI table revision.
    rev: Option<usize>,
    /// ACPI OEM ID override.
    oem_id: Option<ShellString>,
    /// ACPI OEM table ID override.
    oem_table_id: Option<ShellString>,
    /// ACPI OEM revision override.
    oem_rev: Option<usize>,
    /// ASL compiler ID override.
    asl_compiler_id: Option<ShellString>,
    /// ASL compiler revision override.
    asl_compiler_rev: Option<usize>,
    #[proptest(filter = "acpi_table_data_is_nonempty")]
    /// Table content source rendered as either `file=` or `data=`.
    data: Option<AcpiTableData>,
}

fn acpi_table_data_is_nonempty(segment: &Option<AcpiTableData>) -> bool {
    match segment {
        None => false,
        Some(f) => !f.files().is_empty(),
    }
}

impl AcpiTable {
    /// Creates an empty `-acpitable` argument builder for incremental setup.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ToCommand for AcpiTable {
    fn has_args(&self) -> bool {
        self.sig.is_some()
            || self.rev.is_some()
            || self.oem_id.is_some()
            || self.oem_table_id.is_some()
            || self.oem_rev.is_some()
            || self.asl_compiler_id.is_some()
            || self.asl_compiler_rev.is_some()
            || self.data.is_some()
    }
    fn command(&self) -> String {
        ARG_ACPITABLE.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];
        if let Some(sig) = &self.sig {
            args.push(format!("{}{}", KEY_SIG, sig.as_ref()));
        }
        qao!(&self.rev, args, KEY_REV);
        if let Some(oem_id) = &self.oem_id {
            args.push(format!("{}{}", KEY_OEM_ID, oem_id.as_ref()));
        }
        if let Some(oem_table_id) = &self.oem_table_id {
            args.push(format!("{}{}", KEY_OEM_TABLE_ID, oem_table_id.as_ref()));
        }
        qao!(&self.oem_rev, args, KEY_OEM_REV);
        if let Some(asl_compiler_id) = &self.asl_compiler_id {
            args.push(format!("{}{}", KEY_ASL_COMPILER_ID, asl_compiler_id.as_ref()));
        }
        qao!(&self.asl_compiler_rev, args, KEY_ASL_COMPILER_REV);

        if let Some(data) = &self.data {
            args.push(data.to_string());
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AcpiTable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut table = AcpiTable::default();

        for part in s.split(DELIM_COMMA) {
            let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid acpitable option: {part}"))?;
            match key {
                "sig" => table.sig = Some(ShellString::from_str(value)?),
                "rev" => table.rev = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "oem_id" => table.oem_id = Some(ShellString::from_str(value)?),
                "oem_table_id" => table.oem_table_id = Some(ShellString::from_str(value)?),
                "oem_rev" => table.oem_rev = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "asl_compiler_id" => table.asl_compiler_id = Some(ShellString::from_str(value)?),
                "asl_compiler_rev" => table.asl_compiler_rev = Some(value.parse::<usize>().map_err(|e| e.to_string())?),
                "file" => table.data = Some(parse_data_list(value, false)),
                "data" => table.data = Some(parse_data_list(value, true)),
                other => return Err(format!("unsupported acpitable option: {other}")),
            }
        }

        Ok(table)
    }
}

fn parse_data_list(value: &str, is_data: bool) -> AcpiTableData {
    let files = value.split(DELIM_COLON).map(ShellPath::from).collect::<Vec<_>>();
    if is_data { AcpiTableData::Data(files) } else { AcpiTableData::File(files) }
}
