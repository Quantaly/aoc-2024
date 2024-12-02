use anyhow::{anyhow, Result};
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn input_path() -> Result<String> {
    env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("need a command-line argument"))
}

pub fn input_buf_read() -> Result<impl BufRead> {
    Ok(BufReader::new(File::open(input_path()?)?))
}

pub fn input_string() -> Result<String> {
    fs::read_to_string(input_path()?).map_err(Into::into)
}
