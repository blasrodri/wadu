use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    str::FromStr,
};

use crate::{error::WaduError, gist::fetch::load_gist, gist::Gist};

/// Holds the state to reproduce a code snippet.
pub struct Snap {
    identifier: String,
    cargo: String,
    main_rs: String,
}

#[derive(Debug)]
pub enum SnapType {
    Gist,
}

impl FromStr for SnapType {
    type Err = WaduError;

    fn from_str(s: &str) -> Result<Self, WaduError> {
        match s {
            _ => Ok(Self::Gist), // for now, we assume that everything is a Gist.
        }
    }
}

impl From<Gist> for Snap {
    fn from(gist: Gist) -> Self {
        let Gist {
            identifier,
            cargo,
            main_rs,
        } = gist;
        Self {
            cargo,
            main_rs,
            identifier,
        }
    }
}

impl Snap {
    pub fn new(identifier: &str, resource_type: SnapType) -> Result<Self, WaduError> {
        match resource_type {
            SnapType::Gist => Ok(Snap::from(load_gist(identifier)?)),
        }
    }

    pub fn edit(&self, root_path: &str) -> Result<ExitStatus, WaduError> {
        self.setup()?;

        let mut result = Command::new("cargo")
            .arg("run")
            .current_dir(self.root_path(root_path))
            .spawn()?;
        let exit_status = result.wait()?;
        Ok(exit_status)
    }

    pub fn run(&self, root_path: &str) -> Result<ExitStatus, WaduError> {
        self.setup()?;
        let mut result = Command::new("cargo")
            .arg("run")
            .current_dir(self.root_path(root_path))
            .spawn()?;
        let exit_status = result.wait()?;
        Ok(exit_status)
    }

    fn root_path(&self, root_path: &str) -> PathBuf {
        Path::new(root_path).join(self.get_identifier().to_string())
    }
    fn get_identifier(&self) -> &str {
        self.identifier.as_str()
    }

    fn setup(&self) -> Result<(), WaduError> {
        use std::{fs::create_dir_all, path::Path};
        let path = Path::new("/tmp").join(self.get_identifier()).join("src");
        // create dir
        let result = create_dir_all(path.as_path())?;
        self.store_files()?;
        Ok(result)
    }

    fn store_files(&self) -> Result<(), WaduError> {
        let base_path = self.root_path("/tmp");
        let cargo_toml = base_path.join("Cargo.toml");
        let main_rs = base_path.join("src").join("main.rs");
        let mut cargo_toml = File::create(cargo_toml)?;
        cargo_toml.write_all(self.cargo.as_bytes())?;
        let mut main_rs = File::create(main_rs)?;
        main_rs.write_all(self.main_rs.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_snap() {
        let snap = Snap::new("7b4b0a549ff7c15c2aaafa039a992441", SnapType::Gist).unwrap();
        snap.run("/tmp").unwrap();
    }
}
