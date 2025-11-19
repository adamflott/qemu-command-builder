use crate::shell_string::ShellString;
use winnow::Result;
use winnow::ascii::{alphanumeric1, escaped};
use winnow::combinator::preceded;
use winnow::combinator::{alt, terminated};
use winnow::error::ContextError;
use winnow::prelude::*;
use winnow::token::{one_of, take_while};

pub(crate) const DELIM_COMMA: &str = ",";
pub(crate) const DELIM_COLON: &str = ":";

pub(crate) const ARG_NOGRAPHIC: &str = "-nographic";
pub(crate) const ARG_UUID: &str = "-uuid";
pub(crate) const ARG_ENABLE_KVM: &str = "-enable-kvm";
pub(crate) const ARG_NO_USER_CONFIG: &str = "-no-user-config";
pub(crate) const ARG_NODEFAULTS: &str = "-nodefaults";
pub(crate) const ARG_NO_REBOOT: &str = "-no-reboot";
pub(crate) const ARG_NO_SHUTDOWN: &str = "-no-shutdown";
pub(crate) const ARG_DAEMONIZE: &str = "-daemonize";
pub(crate) const ARG_ONLY_MIGRATABLE: &str = "-only-migratable";
pub(crate) const ARG_ENABLE_SYNC_PROFILE: &str = "-enable-sync-profile";
pub(crate) const ARG_USB: &str = "-usb";
pub(crate) const ARG_MEM_PREALLOC: &str = "-mem-prealloc";
pub(crate) const ARG_SNAPSHOT: &str = "-snapshot";
pub(crate) const ARG_FULL_SCREEN: &str = "-full-screen";
pub(crate) const ARG_WIN2K_HACK: &str = "-win2k-hack";
pub(crate) const ARG_NO_FD_BOOTCHK: &str = "-no-fd-bootchk";
pub(crate) const ARG_PRECONFIG: &str = "--preconfig";
pub(crate) const ARG_BIG_S: &str = "-S";
pub(crate) const ARG_LITTLE_S: &str = "-s";
pub(crate) const ARG_XEN_ATTACH: &str = "-xen-attach";
pub(crate) const ARG_XEN_DOMID_RESTRICT: &str = "-xen-domid-restrict";
pub(crate) const ARG_APPEND: &str = "-append";
pub(crate) const ARG_MEM_PATH: &str = "-mem-path";
pub(crate) const ARG_K: &str = "-k";
pub(crate) const ARG_FDA: &str = "-fda";
pub(crate) const ARG_FDB: &str = "-fdb";
pub(crate) const ARG_HDA: &str = "-hda";
pub(crate) const ARG_HDB: &str = "-hdb";
pub(crate) const ARG_HDC: &str = "-hdc";
pub(crate) const ARG_HDD: &str = "-hdd";
pub(crate) const ARG_CDROM: &str = "-cdrom";
pub(crate) const ARG_MTDBLOCK: &str = "-mtdblock";
pub(crate) const ARG_SD: &str = "-sd";
pub(crate) const ARG_BIOS: &str = "-bios";
pub(crate) const ARG_PFLASH: &str = "-pflash";
pub(crate) const ARG_KERNEL: &str = "-kernel";
pub(crate) const ARG_SHIM: &str = "-shim";
pub(crate) const ARG_INITRD: &str = "-initrd";
pub(crate) const ARG_DTB: &str = "-dtb";
pub(crate) const ARG_PIDFILE: &str = "-pidfile";
pub(crate) const ARG_BIG_D: &str = "-D";
pub(crate) const ARG_LITTLE_D: &str = "-d";
pub(crate) const ARG_SEED: &str = "-seed";
pub(crate) const ARG_L: &str = "-L";
pub(crate) const ARG_OPTION_ROM: &str = "-option-rom";
pub(crate) const ARG_LOADVM: &str = "-loadvm";
pub(crate) const ARG_XEN_ID: &str = "-xen-id";
pub(crate) const ARG_DUMP_VMSTATE: &str = "-dump-vmstate";
pub(crate) const ARG_PERFMAP: &str = "-perfmap";
pub(crate) const ARG_JITDUMP: &str = "-jitdump";
pub(crate) const ARG_ECHR: &str = "-echr";
pub(crate) const ARG_READCONFIG: &str = "-readconfig";
pub(crate) const ARG_G: &str = "-g";
pub(crate) const ARG_GDB: &str = "-gdb";

pub(crate) fn non_whitespace_token<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(0.., |c: char| !c.is_whitespace()).parse_next(input)
}

pub(crate) fn all_non_whitespace<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., |c: char| !c.is_whitespace()).parse_next(input)
}

pub(crate) fn ascii_plus_more<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '=' || c == '.' || c == '_' || c == '/').parse_next(input)
}

fn quoted_string(input: &mut &str) -> ModalResult<String> {
    preceded(
        '"', // Matches the opening double quote
        terminated(
            escaped(
                // Characters that can appear in the string without escaping
                // This will match any character that is NOT a double quote or backslash
                one_of(|c| c != '"' && c != '\\'),
                '\\', // The escape character
                alt((
                    // Define the escaped sequences and their results
                    "\\".value('\\'),
                    "\"".value('"'),
                    "n".value('\n'),
                    "r".value('\r'),
                    "t".value('\t'),
                )),
            ),
            '"', // Matches the closing double quote
        ),
    )
    .parse_next(input)
}

fn unquoted_string<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    // Matches one or more alphanumeric characters until a delimiter or end of line.
    // Adjust `till_line_ending` or similar to your specific delimiter (e.g., a comma).
    alphanumeric1.parse_next(input)
}

pub fn optional_quotes_parser(input: &mut &str) -> ModalResult<ShellString> {
    let ss = alt((
        // Try parsing a quoted string first, converting the result to an owned String
        quoted_string,
        // If that fails, try parsing an unquoted string and convert the &str to a String
        unquoted_string.map(|s| s.to_string()),
    ))
    .parse_next(input)?;

    match shellish_parse::parse(&ss, shellish_parse::ParseOptions::default()) {
        Ok(strs) => {
            match ShellString::try_from(strs.join(" ")) {
                Ok(str) => Ok(str),
                Err(tf_err) => {
                    // TODO
                    let cerr = ContextError::new();
                    Err(winnow::error::ErrMode::Cut(cerr))
                }
            }
        }
        Err(err) => {
            // TODO
            let cerr = ContextError::new();
            Err(winnow::error::ErrMode::Cut(cerr))
        }
    }
}
