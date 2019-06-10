use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind, Write};


pub struct Lockfile {
    file_path: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>,
}

impl Lockfile {
    pub fn new(path: &Path) -> Lockfile {
        Lockfile {
            file_path: path.to_path_buf(),
            lock_path: path.with_extension("lock").to_path_buf(),
            lock: None,
        }
    }

    pub fn hold_for_update(&mut self) -> Result<(), std::io::Error> {
        if self.lock.is_none() {
            let open_file = OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(self.lock_path.clone())?;
            
            self.lock = Some(open_file);
        }

        Ok(())
    }

    pub fn write(&mut self, contents: &str) -> Result<(), std::io::Error> {
        self.raise_on_stale_lock()?;

        let mut lock = self.lock.as_ref().unwrap();
        lock.write_all(contents.as_bytes())?;

        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), std::io::Error> {
        self.raise_on_stale_lock()?;
        self.lock = None;
        fs::rename(self.lock_path.clone(), self.file_path.clone())?;

        Ok(())
    }

    fn raise_on_stale_lock(&self) -> Result<(), std::io::Error> {
        if self.lock.is_none() {
            Err(io::Error::new(ErrorKind::Other,
                               format!("Not holding lock on file: {:?}", self.lock_path)))
        } else {
            Ok(())
        }
    }
}
