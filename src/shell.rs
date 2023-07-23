use crate::{
    errors::{Error, Result},
    infer::Infer,
};
use std::str::FromStr;

impl Shell {
    pub fn infer() -> Result<Self> {
        Infer::infer()
    }
}

macro_rules! define_shells {
    ($( $name:literal $(| $alias:literal ),* => $shell:tt ),*) => {
        #[derive(Debug)]
        pub enum Shell {
            $(
                $shell,
            )*
        }

        impl FromStr for Shell {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $name $(| $alias)* => Ok(Self::$shell),
                    )*
                    _ => Err(Error::NoShellFound),
                }
            }
        }

        impl ToString for Shell {
            fn to_string(&self) -> String {
                match self {
                    $(
                        Self::$shell => $name.to_string(),
                    )*
                }
            }
        }
    };
}

define_shells!(
    "sh" => Sh,
    "bash" => Bash,
    "zsh" =>  Zsh,
    "fish" =>  Fish,
    "powershell" | "pwsh" => PowerShell,
    "cmd" => Cmd
);

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    fn integrate() {
        let actual = env::var("ACTUAL_SHELL").expect("env var \"ACTUAL_SHELL\" is not specified");

        let res = Shell::infer().unwrap();
        assert_eq!(actual, res.to_string());
    }
}
