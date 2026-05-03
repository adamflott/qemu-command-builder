use crate::parsers::DELIM_COMMA;
use crate::to_command::ToCommand;
use bon::Builder;
use proptest_derive::Arbitrary;
use std::str::FromStr;

pub(crate) const ARG_OBJECT: &str = "-objectfd";

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default, Builder, Arbitrary)]
pub struct Object {
    typename: String,
    properties: Vec<(String, String)>,
}

impl Object {
    pub fn new<S: AsRef<str>>(typename: S) -> Self {
        Object {
            typename: typename.as_ref().to_string(),
            properties: Default::default(),
        }
    }
    pub fn add_prop<S: AsRef<str>>(&mut self, key: S, value: S) -> &mut Self {
        self.properties.push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
}

impl ToCommand for Object {
    fn command(&self) -> String {
        ARG_OBJECT.to_string()
    }
    fn to_args(&self) -> Vec<String> {
        let mut args = vec![self.typename.clone()];

        for (prop_key, prop_value) in &self.properties {
            args.push(format!("{}={}", prop_key, prop_value));
        }
        vec![args.join(DELIM_COMMA)]
    }
}

impl FromStr for Object {
    type Err = ();

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
