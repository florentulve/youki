use std::ffi::CString;

use anyhow::{bail, Context, Result};
use nix::unistd;
use oci_spec::runtime::Spec;

use super::{ExecHandler, EMPTY};

pub struct DefaultExecHandler {}

impl ExecHandler for DefaultExecHandler {
    fn exec(&self, spec: &Spec) -> Result<()> {
        log::debug!("Executing workload with default handler");
        let args = spec
            .process()
            .as_ref()
            .and_then(|p| p.args().as_ref())
            .unwrap_or_else(|| &EMPTY);

        if args.len() == 0 {
            bail!("at least one process arg must be specified")
        }

        let executable = args[0].as_str();
        let p = CString::new(executable.as_bytes())
            .with_context(|| format!("failed to convert path {:?} to cstring", executable))?;
        let a: Vec<CString> = args
            .iter()
            .map(|s| CString::new(s.as_bytes()).unwrap_or_default())
            .collect();
        unistd::execvp(&p, &a)?;

        // After do_exec is called, the process is replaced with the container
        // payload through execvp, so it should never reach here.
        unreachable!();
    }

    fn can_handle(&self, _: &Spec) -> Result<bool> {
        Ok(true)
    }

    fn name(&self) -> &str {
        "default"
    }
}

