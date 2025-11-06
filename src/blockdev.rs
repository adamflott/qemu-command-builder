use std::collections::BTreeMap;

use bon::Builder;

use crate::common::{IgnoreUnmap, OnOff, OnOffUnmap};
use crate::to_command::{ToArg, ToCommand};

/// Define a new block driver node. Some of the options apply to all
/// block drivers, other options are only accepted for a specific block
/// driver. See below for a list of generic options and options for the
/// most common block drivers.
///
/// Options that expect a reference to another node (e.g. ``file``) can
/// be given in two ways. Either you specify the node name of an already
/// existing node (file=node-name), or you define a new node inline,
/// adding options for the referenced node after a dot
/// (file.filename=path,file.aio=native).
///
/// A block driver node created with ``-blockdev`` can be used for a
/// guest device by specifying its node name for the ``drive`` property
/// in a ``-device`` argument that defines a block device.
///
/// TODO
/// - constrain driver opts
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Builder)]
pub struct BlockDev {
    /// Specifies the block driver to use for the given node.
    pub driver: String,

    /// This defines the name of the block driver node by which it
    /// will be referenced later. The name must be unique, i.e. it
    /// must not match the name of a different block driver node, or
    /// (if you use ``-drive`` as well) the ID of a drive.
    ///
    /// If no node name is specified, it is automatically generated.
    /// The generated node name is not intended to be predictable
    /// and changes between QEMU invocations. For the top level, an
    /// explicit node name must be specified.
    pub node_name: Option<String>,

    /// discard is one of "ignore" (or "off") or "unmap" (or "on")
    /// and controls whether ``discard`` (also known as ``trim`` or
    /// ``unmap``) requests are ignored or passed to the filesystem.
    /// Some machine types may not support discard requests.
    pub discard: Option<IgnoreUnmap>,

    /// The host page cache can be avoided with ``cache.direct=on``.
    /// This will attempt to do disk IO directly to the guest's
    /// memory. QEMU may still perform an internal copy of the data.
    pub cache_direct: Option<OnOff>,

    /// In case you don't care about data integrity over host
    /// failures, you can use ``cache.no-flush=on``. This option
    /// tells QEMU that it never needs to write any data to the disk
    /// but can instead keep things in cache. If anything goes
    /// wrong, like your host losing power, the disk storage getting
    /// disconnected accidentally, etc. your image will most
    /// probably be rendered unusable.
    pub cache_no_flush: Option<OnOff>,

    /// Open the node read-only. Guest write attempts will fail.
    ///
    /// Note that some block drivers support only read-only access,
    /// either generally or in certain configurations. In this case,
    /// the default value ``read-only=off`` does not work and the
    /// option must be specified explicitly.
    pub read_only: Option<OnOff>,

    /// If ``auto-read-only=on`` is set, QEMU may fall back to
    /// read-only usage even when ``read-only=off`` is requested, or
    /// even switch between modes as needed, e.g. depending on
    /// whether the image file is writable or whether a writing user
    /// is attached to the node.
    pub auto_read_only: Option<OnOff>,

    /// Override the image locking system of QEMU by forcing the
    /// node to utilize weaker shared access for permissions where
    /// it would normally request exclusive access. When there is
    /// the potential for multiple instances to have the same file
    /// open (whether this invocation of QEMU is the first or the
    /// second instance), both instances must permit shared access
    /// for the second instance to succeed at opening the file.
    ///
    /// Enabling ``force-share=on`` requires ``read-only=on``.
    pub force_share: Option<OnOff>,

    /// detect-zeroes is "off", "on" or "unmap" and enables the
    /// automatic conversion of plain zero writes by the OS to
    /// driver specific optimized zero write commands. You may even
    /// choose "unmap" if discard is set to "unmap" to allow a zero
    /// write to be converted to an ``unmap`` operation.
    pub detect_zeroes: Option<OnOffUnmap>,

    pub driver_opts: Option<BTreeMap<String, String>>,
}

impl ToCommand for BlockDev {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec![];

        cmd.push("-blockdev".to_string());

        let mut args = vec![];

        args.push(format!("driver={}", self.driver));
        if let Some(node_name) = &self.node_name {
            args.push(format!("node-name={}", node_name));
        }
        if let Some(discard) = &self.discard {
            args.push(format!("discard={}", discard.to_arg()));
        }
        if let Some(cache_direct) = &self.cache_direct {
            args.push(format!("cache.direct={}", cache_direct.to_arg()));
        }
        if let Some(cache_no_flush) = &self.cache_no_flush {
            args.push(format!("cache.no-flush={}", cache_no_flush.to_arg()));
        }
        if let Some(read_only) = &self.read_only {
            args.push(format!("read-only={}", read_only.to_arg()));
        }
        if let Some(auto_read_only) = &self.auto_read_only {
            args.push(format!("auto-read-only={}", auto_read_only.to_arg()));
        }
        if let Some(force_share) = &self.force_share {
            args.push(format!("force-share={}", force_share.to_arg()));
        }
        if let Some(detect_zeroes) = &self.detect_zeroes {
            args.push(format!("detect-zeroes={}", detect_zeroes.to_arg()));
        }
        if let Some(driver_opts) = &self.driver_opts {
            for (key, val) in driver_opts {
                args.push(format!("{}={}", key, val));
            }
        }

        cmd.push(args.join(","));

        cmd
    }
}
