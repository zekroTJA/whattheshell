use crate::{
    errors::{Error, Result},
    infer::Infer,
};
use core::fmt;
use std::str::FromStr;

impl Shell {
    /// Tries to infer the shell in which the program is currently
    /// running in from the environment.
    ///
    /// ## Example
    /// ```
    /// # use whattheshell::Shell;
    /// let shell = Shell::infer().unwrap();
    /// dbg!(shell);
    /// ```
    pub fn infer() -> Result<Self> {
        Infer::infer()
    }
}

macro_rules! define_shells {
    ($( $name:literal $(| $alias:literal ),* => $shell:tt ),*) => {
        /// Shell declares a type of shell and allows to
        /// infer the type of shell.
        #[derive(Clone, Debug)]
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

        impl fmt::Display for Shell {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(
                        Self::$shell => write!(f, $name),
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
    use super::*;
    use std::env;

    #[test]
    fn integrate() {
        let actual = env::var("ACTUAL_SHELL").expect("env var \"ACTUAL_SHELL\" is not specified");

        let res = Shell::infer().unwrap();
        assert_eq!(actual, res.to_string());
    }
}
