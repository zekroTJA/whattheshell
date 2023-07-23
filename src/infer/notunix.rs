#![cfg(not(unix))]

use super::Infer;
use crate::{
    errors::{Error, Result},
    shell::Shell,
};
use sysinfo::{ProcessExt, System, SystemExt};

impl Infer {
    pub fn infer() -> Result<Shell> {
        let mut system = System::new();
        let mut pid = Some(sysinfo::get_current_pid().map_err(Error::FailedGettingPid)?);

        while let Some(current_pid) = pid {
            system.refresh_process(current_pid);
            if let Some(process) = system.process(current_pid) {
                pid = process.parent();
                let name = process
                    .exe()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase());
                if let Some(shell) = name.and_then(|n| n.parse().ok()) {
                    return Ok(shell);
                }
            }
        }

        Err(Error::NoShellFound)
    }
}
