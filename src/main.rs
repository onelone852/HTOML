mod arg;
mod error;
mod html;
mod htoml;

use error::{HtomlError, Result};
use htoml::Htoml;

fn main() -> Result<()> {
    let toml = std::fs::read_to_string("test.toml").or(Err(HtomlError::ReadFileError))?;
    let htoml = Htoml::new(toml)?;
    let output = htoml.parse()?;
    std::fs::write("test.html", output).or(Err(HtomlError::WriteFileError))?;
    Ok(())
}
