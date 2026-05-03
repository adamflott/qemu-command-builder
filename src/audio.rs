use std::str::FromStr;

use crate::parsers::DELIM_COMMA;
use crate::qao;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;

pub(crate) const ARG_AUDIO: &str = "-audio";

const KEY_DRIVER: &str = "driver=";
const KEY_MODEL: &str = "model=";

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
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Audio {
    driver: String,
    model: Option<String>,
    properties: Vec<(String, String)>,
}

impl Audio {
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties.push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
}

impl ToCommand for Audio {
    fn command(&self) -> String {
        ARG_AUDIO.to_string()
    }

    fn to_args(&self) -> Vec<String> {
        let mut args = vec![format!("{}{}", KEY_DRIVER, self.driver.to_string())];

        qao!(&self.model, args, KEY_MODEL);
        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }

        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Audio {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
