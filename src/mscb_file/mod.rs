mod parser;
mod writer;

use super::Script;
use parser::take_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct MscsbFile {
    pub scripts: Vec<Script>,
    pub strings: Vec<String>,
    pub entrypoint: u32,
}

impl MscsbFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Option<MscsbFile> {
        let mut buffer = vec![];
        File::open(path).ok()?.read_to_end(&mut buffer).ok()?;
        Some(take_file(&buffer[..]).unwrap().1)
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut buffer = vec![];
        self.write(&mut buffer);
        File::create(path)?
            .write_all(&buffer[..])?;
        Ok(())
    }

    pub fn iter(&self) -> std::slice::Iter<Script> {
        self.scripts.iter()
    }
}

