mod arg;
mod command;
mod error;
mod html;
mod htoml;

use command::Command;
use error::Result;

fn main() -> Result<()> {
    Command::new()?.run()
}
