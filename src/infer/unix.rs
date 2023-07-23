#![cfg(unix)]

use crate::{
    errors::{Error, ProcessInfoError, Result},
    shell::Shell,
};
use std::{
    io::{BufRead, BufReader},
    process::{self, Command, Stdio},
};

use super::Infer;

const MAX_DEPTH: u8 = 30;

struct ProcessInfo {
    ppid: Option<u32>,
    command: String,
}

impl Infer {
    pub fn infer() -> Result<Shell> {
        let mut pid = Some(process::id());
        let mut depth = 0;

        while let Some(current_pid) = pid {
            if depth > MAX_DEPTH {
                return Err(Error::NoShellFound);
            }

            let proc_info = get_process_info(current_pid)?;
            let filename = proc_info
                .command
                .trim_start_matches('-')
                .split('/')
                .last()
                .ok_or_else(|| Error::NoBinary(proc_info.command.clone()))?;

            if let Ok(shell) = filename.parse() {
                return Ok(shell);
            }

            pid = proc_info.ppid;
            depth += 1;
        }

        Err(Error::NoShellFound)
    }
}

fn get_process_info(pid: u32) -> Result<ProcessInfo, ProcessInfoError> {
    let stdout = Command::new("ps")
        .args(["-o", "ppid,comm", pid.to_string().as_str()])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| ProcessInfoError::UnexpectedEof)?;

    let mut stdout_lines = BufReader::new(stdout).lines();

    let line = stdout_lines
        .nth(1)
        .transpose()?
        .ok_or_else(|| ProcessInfoError::UnexpectedEof)?;

    let mut parts = line.split_whitespace();
    let ppid = parts
        .next()
        .ok_or_else(|| ProcessInfoError::UnexpectedOutput {
            msg: "ppid should be the first item in table",
            output: line.to_string(),
        })?;
    let command = parts
        .next()
        .ok_or_else(|| ProcessInfoError::UnexpectedOutput {
            msg: "command should be the second item in table",
            output: line.to_string(),
        })?;

    Ok(ProcessInfo {
        ppid: ppid.parse().ok(),
        command: command.to_string(),
    })
}
