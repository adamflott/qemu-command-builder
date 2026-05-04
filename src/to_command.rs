use proptest::prelude::Arbitrary;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

fn shell_escape_arg(arg: &str) -> String {
    if !arg.is_empty() && arg.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':' | ',' | '=' | '+')) {
        return arg.to_string();
    }

    let escaped = arg.replace('\'', r#"'\''"#);
    format!("'{}'", escaped)
}

pub trait ToCommand: Debug + Clone + Hash + Ord + PartialOrd + Eq + PartialEq + FromStr + Arbitrary {
    fn has_args(&self) -> bool {
        true
    }

    /// Convert to a type suitable to pass to [`std::process::Command::new()`]
    fn command(&self) -> String {
        String::new()
    }

    /// Convert to a type suitable to pass to [`std::process::Command::args()`]
    fn to_args(&self) -> Vec<String>;

    /// Construct the full command in keep in pieces
    fn to_command(&self) -> Vec<String> {
        if self.has_args() {
            let mut cmd = vec![self.command()];
            cmd.append(&mut self.to_args());
            cmd
        } else {
            vec![]
        }
    }

    /// Construct the full command as a single [`String`]
    fn to_single_command(&self) -> String {
        self.to_command().iter().map(|arg| shell_escape_arg(arg)).collect::<Vec<_>>().join(" ")
    }

    /// Construct only the args as a single [`String`]
    fn to_single_arg(&self) -> String {
        self.to_args().iter().map(|arg| shell_escape_arg(arg)).collect::<Vec<_>>().join(" ")
    }
}

pub trait ToArg {
    fn to_arg(&self) -> &str;
}

// Quick Add Optional
#[macro_export]
macro_rules! qao {
    ($a:expr,$b:expr,$c:expr) => {{
        if let Some(val) = $a {
            $b.push(format!("{}{}", $c, val));
        }
    }};
}

// Quick Add Optional OsString
#[macro_export]
macro_rules! qaoo {
    ($a:expr,$b:expr,$c:expr) => {
        if let Some(val) = $a {
            match val.clone().into_string() {
                Ok(str) => $b.push(format!("{}{}", $c, str)),
                Err(x) => panic!("can't convert {:?} to a string", x),
            }
        }
    };
}

// Quick Add Optional Path
#[macro_export]
macro_rules! qaop {
    ($a:expr,$b:expr,$c:expr) => {{
        if let Some(val) = $a {
            $b.push(format!("{}{}", $c, val.display()));
        }
    }};
}

// Parse Custom (Type) Option
#[macro_export]
macro_rules! pco {
    ($a:ident,$b:expr,$c:ty,$d:expr) => {
        fn $a(s: &mut &str) -> ModalResult<$c> {
            let _ = literal(DELIM_COMMA).parse_next(s)?;
            let _ = literal($d).parse_next(s)?;
            let parsed = $b.parse_to::<$c>().parse_next(s)?;
            Ok(parsed)
        }
    };
}
// Parse Custom (Type) Option with optional leading comma
#[macro_export]
macro_rules! pco0 {
    ($a:ident,$b:expr,$c:ty,$d:expr) => {
        fn $a(s: &mut &str) -> ModalResult<$c> {
            let _ = opt(literal(DELIM_COMMA)).parse_next(s)?;
            let _ = literal($d).parse_next(s)?;
            let parsed = $b.parse_to::<$c>().parse_next(s)?;
            Ok(parsed)
        }
    };
}
// Parse Primitive (Type) Option
#[macro_export]
macro_rules! ppo {
    ($a:ident,$b:expr,$c:ty,$d:expr) => {
        fn $a(s: &mut &str) -> ModalResult<$c> {
            let _ = literal(DELIM_COMMA).parse_next(s)?;
            let _ = literal($d).parse_next(s)?;
            let parsed = $b.parse_next(s)?;
            Ok(parsed)
        }
    };
}

// Parse Primitive (Type) Option with optional leading comma
#[macro_export]
macro_rules! ppo0 {
    ($a:ident,$b:expr,$c:ty,$d:expr) => {
        fn $a(s: &mut &str) -> ModalResult<$c> {
            let _ = opt(literal(DELIM_COMMA)).parse_next(s)?;
            let _ = literal($d).parse_next(s)?;
            let parsed = $b.parse_next(s)?;
            Ok(parsed)
        }
    };
}

// Parse String (Type) Option
#[macro_export]
macro_rules! pso {
    ($a:ident,$b:expr) => {
        fn $a(s: &mut &str) -> ModalResult<ShellString> {
            let _ = literal(DELIM_COMMA).parse_next(s)?;
            let _ = literal($b).parse_next(s)?;
            let parsed = ascii_plus_more.parse_next(s)?;
            Ok(ShellString { s: parsed.to_string() })
        }
    };
}

// Parse String (Type) Option with optional leading comma
#[macro_export]
macro_rules! pso0 {
    ($a:ident,$b:expr) => {
        fn $a(s: &mut &str) -> ModalResult<ShellString> {
            let _ = opt(literal(DELIM_COMMA)).parse_next(s)?;
            let _ = literal($b).parse_next(s)?;
            let parsed = ascii_plus_more.parse_next(s)?;
            Ok(ShellString { s: parsed.to_string() })
        }
    };
}
