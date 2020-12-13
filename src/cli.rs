use std::process::ExitStatus;

use crate::{
    error::WaduError,
    runner::Runner,
    snap::{self, SnapType},
};
use structopt::StructOpt;

/// Wadu: share Rust easily
#[derive(StructOpt, Debug)]
#[structopt(name = "Wadu")]
struct WaduOpt {
    pub identifier: String,

    #[structopt(long, short, default_value = "gist")]
    pub snap_type: SnapType,
}

pub fn start_cli() -> Result<ExitStatus, WaduError> {
    let WaduOpt { ref identifier, .. } = WaduOpt::from_args();
    let snap = snap::Snap::new(identifier, SnapType::Gist)?;
    snap.run("/tmp")
}
