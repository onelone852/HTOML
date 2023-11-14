use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::htoml::Htoml;

use super::error::{HtomlError, Result};

pub enum Command {
    Compile { path: PathBuf },
}

enum CommandType {
    TCompile,
}

impl FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use CommandType::*;
        match s {
            "compile" | "cp" | "c" => Ok(TCompile),
            _ => Err(s.to_string()),
        }
    }
}

impl Command {
    pub fn new() -> Result<Self> {
        use Command::*;
        use CommandType::*;
        let args: Vec<String> = std::env::args().collect();
        let command_type = args
            .get(1)
            .ok_or(HtomlError::NoCommand)?
            .parse::<CommandType>()
            .or_else(|s| Err(HtomlError::UnknownCommand(s)))?;
        match command_type {
            TCompile => {
                let file = args.get(2).ok_or(HtomlError::NoFileGiven)?;
                Ok(Compile {
                    path: Path::new(file).to_path_buf(),
                })
            }
        }
    }

    pub fn run(self) -> Result<()> {
        use Command::*;
        match self {
            Compile { mut path } => {
                let toml = std::fs::read_to_string(&path).or(Err(HtomlError::ReadFileError))?;
                let htoml = Htoml::new(toml)?;
                let output = htoml.parse()?;
                path.set_extension("html");
                std::fs::write(path, output).or(Err(HtomlError::WriteFileError))?;
                Ok(())
            }
        }
    }
}
