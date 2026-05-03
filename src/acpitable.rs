use std::str::FromStr;

use crate::parsers::{DELIM_COLON, DELIM_COMMA};
use crate::shell_path::{ShellPath, shell_path_until_colon};
use crate::shell_string::{ShellString, ShellStringError, shell_string_until_comma};
use crate::to_command::ToCommand;
use crate::{pco0, ppo0, qao};
use bon::Builder;
use proptest_derive::Arbitrary;
use winnow::Result;
use winnow::ascii::dec_uint;
use winnow::combinator::{opt, separated};
use winnow::prelude::*;
use winnow::token::literal;

pub(crate) const ARG_ACPITABLE: &str = "-acpitable";

const KEY_SIG: &str = "sig=";
const KEY_REV: &str = "rev=";
const KEY_OEM_ID: &str = "oem_id=";
const KEY_OEM_TABLE_ID: &str = "oem_table_id=";
const KEY_OEM_REV: &str = "oem_rev=";
const KEY_ASL_COMPILER_ID: &str = "asl_compiler_id=";
const KEY_ASL_COMPILER_REV: &str = "asl_compiler_rev=";
const KEY_FILE: &str = "file=";

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
    sig: Option<ShellString>,
    rev: Option<usize>,
    oem_id: Option<ShellString>,
    oem_table_id: Option<ShellString>,
    oem_rev: Option<usize>,
    asl_compiler_id: Option<ShellString>,
    asl_compiler_rev: Option<usize>,
    #[proptest(filter = "is_nonempty")]
    data: Option<Vec<ShellPath>>,
}

fn is_nonempty(segment: &Option<Vec<ShellPath>>) -> bool {
    match segment {
        None => false,
        Some(f) => !f.is_empty(),
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
            let files: Vec<&str> = data.iter().map(|p| p.as_ref()).collect();
            args.push(format!("{}{}", KEY_FILE, files.join(DELIM_COLON)));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for AcpiTable {
    type Err = ShellStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        acpitable.parse(s).map_err(|e| ShellStringError::from_parse(e))
    }
}

pco0!(sig, shell_string_until_comma, ShellString, KEY_SIG);
ppo0!(rev, dec_uint, usize, KEY_REV);
pco0!(oem_id, shell_string_until_comma, ShellString, KEY_OEM_ID);
pco0!(oem_table_id, shell_string_until_comma, ShellString, KEY_OEM_TABLE_ID);
ppo0!(oem_rev, dec_uint, usize, KEY_OEM_REV);
pco0!(asl_compiler_id, shell_string_until_comma, ShellString, KEY_ASL_COMPILER_ID);
ppo0!(asl_compiler_rev, dec_uint, usize, KEY_ASL_COMPILER_REV);

fn data(s: &mut &str) -> ModalResult<Vec<ShellPath>> {
    opt(literal(DELIM_COMMA)).parse_next(s)?;
    let _ = literal(KEY_FILE).parse_next(s)?;
    let str: Vec<&str> = separated(1.., shell_path_until_colon, DELIM_COLON).parse_next(s)?;
    let str = str.iter().map(|v| ShellPath { s: v.to_string() }).collect();
    Ok(str)
}

pub fn acpitable(s: &mut &str) -> ModalResult<AcpiTable> {
    let sig = opt(sig).parse_next(s)?;
    let rev = opt(rev).parse_next(s)?;
    let oem_id = opt(oem_id).parse_next(s)?;
    let oem_table_id = opt(oem_table_id).parse_next(s)?;
    let oem_rev = opt(oem_rev).parse_next(s)?;
    let asl_compiler_id = opt(asl_compiler_id).parse_next(s)?;
    let asl_compiler_rev = opt(asl_compiler_rev).parse_next(s)?;
    let data = opt(data).parse_next(s)?;
    Ok(AcpiTable {
        sig,
        rev,
        oem_id,
        oem_table_id,
        oem_rev,
        asl_compiler_id,
        asl_compiler_rev,
        data,
    })
}
