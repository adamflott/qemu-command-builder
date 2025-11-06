use bon::Builder;

use crate::to_command::ToCommand;

/// Add a file descriptor to an fd set.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct AddFd {
    /// This option defines the file descriptor of which a duplicate is
    /// added to fd set. The file descriptor cannot be stdin, stdout, or
    /// stderr.
    fd: usize,
    /// This option defines the ID of the fd set to add the file
    /// descriptor to.
    set: usize,
    /// This option defines a free-form string that can be used to
    /// describe fd.
    opaque: Option<String>,
}

impl ToCommand for AddFd {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-add-fd".to_string()];

        let mut arg = vec![self.fd.to_string()];
        arg.push(format!(",set={}", self.set));
        if let Some(opaque) = &self.opaque {
            arg.push(format!(",opaque={}", opaque));
        }
        cmd.push(arg.join(","));
        cmd
    }
}
