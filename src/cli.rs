use std::process::ExitStatus;

use crate::{
    error::WaduError,
    snap::{self, SnapType},
};
use structopt::StructOpt;

/// Wadu: share Rust code with ease
#[derive(StructOpt, Debug)]
#[structopt(name = "Wadu")]
struct WaduOpt {
    pub identifier: String,

    #[structopt(long, short, default_value = "gist")]
    pub snap_type: SnapType,

    #[structopt(long, short)]
    pub edit: Option<bool>,
}

pub fn start_cli() -> Result<ExitStatus, WaduError> {
    let WaduOpt {
        ref identifier,
        edit,
        ..
    } = WaduOpt::from_args();
    let snap = snap::Snap::new(identifier, SnapType::Gist)?;
    if let Some(true) = edit {
        return snap.edit("/tmp");
    }
    snap.run("/tmp")
}
