use bon::Builder;

use crate::to_command::ToCommand;

/// If the ``model`` option is specified, ``-audio`` is a shortcut
/// for configuring both the guest audio hardware and the host audio
/// backend in one go. The guest hardware model can be set with
/// ``model=modelname``.  Use ``model=help`` to list the available
/// device types.
///
/// The following two example do exactly the same, to show how ``-audio``
/// can be used to shorten the command line length:
///
///
/// -audiodev pa,id=pa -device sb16,audiodev=pa
/// -audio pa,model=sb16
///
/// If the ``model`` option is not specified, ``-audio`` is used to
/// configure a default audio backend that will be used whenever the
/// ``audiodev`` property is not set on a device or machine.  In
/// particular, ``-audio none`` ensures that no audio is produced even
/// for machines that have embedded sound hardware.
///
/// In both cases, the driver option is the same as with the corresponding
/// ``-audiodev`` option below.  Use ``driver=help`` to list the available
/// drivers.
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder)]
pub struct Audio {
    driver: String,
    model: Option<String>,
    property: String,
}

impl ToCommand for Audio {
    fn to_command(&self) -> Vec<String> {
        let mut cmd = vec!["-audio".to_string()];

        let mut args = vec![format!("driver={}", self.driver.to_string())];
        if let Some(model) = &self.model {
            args.push(format!(",model={}", model));
        }
        args.push(format!(",prop={}", self.property));

        cmd.push(args.join(","));
        cmd
    }
}
