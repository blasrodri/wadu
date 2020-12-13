use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use crate::error::WaduError;
/// Runner takes a `Snap` and runs the code
pub trait Runner {
    fn run(&self, root_path: &str) -> Result<ExitStatus, WaduError> {
        self.setup()?;
        let mut result = Command::new("cargo")
            .arg("run")
            .current_dir(self.root_path(root_path))
            .spawn()?;
        let exit_status = result.wait()?;
        Ok(exit_status)
    }

    fn setup(&self) -> Result<(), WaduError> {
        let path = Path::new("/tmp").join(self.get_identifier()).join("src");
        // create dir
        let result = create_dir_all(path.as_path())?;
        self.store_files()?;
        Ok(result)
    }

    fn store_files(&self) -> Result<(), WaduError>;

    fn get_identifier(&self) -> &str;
    fn root_path(&self, root_path: &str) -> PathBuf {
        Path::new(root_path).join(self.get_identifier().to_string())
    }
}
