mod parser;

use super::{Script, Command, Cmd};
use parser::take_file;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct MscbFile {
    scripts: Vec<Script>,
    strings: Vec<String>,
    entrypoint: u32,
}

impl MscbFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Option<MscbFile> {
        let mut buffer = Vec::new();
        File::open(path).ok()?.read_to_end(&mut buffer).ok()?;
        Some(take_file(&buffer[..]).ok()?.1)
    }
}
