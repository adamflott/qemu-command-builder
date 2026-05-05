use crate::args::fsdev::SecurityModel;
use crate::parsers::{ARG_VIRTFS, DELIM_COMMA};
use crate::to_command::{ToArg, ToCommand};
use bon::Builder;
use proptest_derive::Arbitrary;
use std::path::PathBuf;
use std::str::FromStr;

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
    id: Option<String>,
    readonly: Option<bool>,
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
                args.push(format!("security_model={}", local.security_mode.to_arg()));
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
                if let Some(id) = &synth.id {
                    args.push(format!("id={}", id));
                }
                if let Some(readonly) = &synth.readonly
                    && *readonly
                {
                    args.push("readonly=on".to_string());
                }
            }
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Virtfs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(DELIM_COMMA);
        let backend = parts.next().ok_or_else(|| "empty -virtfs argument".to_string())?;
        match backend {
            "local" => {
                let mut path = None;
                let mut mount_tag = None;
                let mut security_mode = None;
                let mut id = None;
                let mut writeout = None;
                let mut readonly = None;
                let mut fmode = None;
                let mut dmode = None;
                let mut multidevs = None;

                for part in parts {
                    let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid virtfs local option: {part}"))?;
                    match key {
                        "path" => path = Some(PathBuf::from(value)),
                        "mount_tag" => mount_tag = Some(value.to_string()),
                        "security_model" => security_mode = Some(value.parse::<SecurityModel>().map_err(|e| e.to_string())?),
                        "id" => id = Some(value.to_string()),
                        "writeout" => {
                            if value != "immediate" {
                                return Err(format!("invalid writeout value: {value}"));
                            }
                            writeout = Some(());
                        }
                        "readonly" => {
                            if value != "on" {
                                return Err(format!("invalid readonly value: {value}"));
                            }
                            readonly = Some(true);
                        }
                        "fmode" => fmode = Some(value.to_string()),
                        "dmode" => dmode = Some(value.to_string()),
                        "multidevs" => {
                            multidevs = Some(match value {
                                "remap" => RemapForbidWarn::Remap,
                                "forbid" => RemapForbidWarn::Forbid,
                                "warn" => RemapForbidWarn::Warn,
                                _ => return Err(format!("invalid multidevs value: {value}")),
                            })
                        }
                        other => return Err(format!("unsupported virtfs local option: {other}")),
                    }
                }

                Ok(Self::Local(Local {
                    path: path.ok_or_else(|| "virtfs local requires path=".to_string())?,
                    mount_tag: mount_tag.ok_or_else(|| "virtfs local requires mount_tag=".to_string())?,
                    security_mode: security_mode.ok_or_else(|| "virtfs local requires security_model=".to_string())?,
                    id,
                    writeout,
                    readonly,
                    fmode,
                    dmode,
                    multidevs,
                }))
            }
            "synth" => {
                let mut mount_tag = None;
                let mut id = None;
                let mut readonly = None;
                for part in parts {
                    let (key, value) = part.split_once('=').ok_or_else(|| format!("invalid virtfs synth option: {part}"))?;
                    match key {
                        "mount_tag" => mount_tag = Some(value.to_string()),
                        "id" => id = Some(value.to_string()),
                        "readonly" => {
                            if value != "on" {
                                return Err(format!("invalid readonly value: {value}"));
                            }
                            readonly = Some(true);
                        }
                        other => return Err(format!("unsupported virtfs synth option: {other}")),
                    }
                }
                Ok(Self::Synth(Synth {
                    mount_tag: mount_tag.ok_or_else(|| "virtfs synth requires mount_tag=".to_string())?,
                    id,
                    readonly,
                }))
            }
            other => Err(format!("unsupported virtfs backend: {other}")),
        }
    }
}
