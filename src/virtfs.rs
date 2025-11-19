use crate::fsdev::SecurityModel;
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) const ARG_VIRTFS: &str = "-virtfs";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum RemapForbidWarn {
    Remap,
    Forbid,
    Warn,
}

impl ToArg for RemapForbidWarn {
    fn to_arg(&self) -> &str {
        match self {
            RemapForbidWarn::Remap => "remap",
            RemapForbidWarn::Forbid => "forbid",
            RemapForbidWarn::Warn => "warn",
        }
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Local {
    path: PathBuf,
    mount_tag: String,
    security_mode: SecurityModel,
    id: Option<String>,
    writeout: Option<()>,
    readonly: Option<bool>,
    fmode: Option<String>,
    dmode: Option<String>,
    multidevs: Option<RemapForbidWarn>,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder, Arbitrary)]
pub struct Synth {
    mount_tag: String,
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Arbitrary)]
pub enum Virtfs {
    Local(Local),
    Synth(Synth),
}
impl ToCommand for Virtfs {
    fn command(&self) -> String {
        ARG_VIRTFS.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self {
            Virtfs::Local(local) => {
                args.push("local".to_string());
                args.push(format!("path={}", local.path.display()));
                args.push(format!("mount_tag={}", local.mount_tag));
                args.push(format!("security_mode={}", local.security_mode.to_arg()));
                if let Some(id) = &local.id {
                    args.push(format!("id={}", id));
                }
                if local.writeout.is_some() {
                    args.push("writeout=immediate".to_string());
                }
                if let Some(readonly) = &local.readonly
                    && *readonly
                {
                    args.push("readonly=on".to_string());
                }
                if let Some(fmode) = &local.fmode {
                    args.push(format!("fmode={}", fmode));
                }
                if let Some(dmode) = &local.dmode {
                    args.push(format!("dmode={}", dmode));
                }
                if let Some(multidevs) = &local.multidevs {
                    args.push(format!("multidevs={}", multidevs.to_arg()));
                }
            }
            Virtfs::Synth(synth) => {
                args.push("synth".to_string());
                args.push(format!("mount_tag={}", synth.mount_tag));
            }
        }

        args
    }
}

impl FromStr for Virtfs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
