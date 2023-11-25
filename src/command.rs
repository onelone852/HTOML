use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use super::{
    error::{HtomlError, Result},
    htoml::Htoml,
};

pub enum Command {
    Compile { path: PathBuf },
    Help,
}

enum CommandType {
    TCompile,
    THelp,
}

impl FromStr for CommandType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use CommandType::*;
        match s {
            "compile" | "cp" | "c" => Ok(TCompile),
            "h" | "help" => Ok(THelp),
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
            THelp => Ok(Help),
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
            Help => {
                println!("Usage: htoml [command] [args...]\nCommands:\n h / help - Show this help message\n c / cp / compile - Compile a valid TOML file to HTML\n  Args:\n   arg1： File that is going to be compiled");
                Ok(())
            }
        }
    }
}
