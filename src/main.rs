mod commands;
mod helpers;
mod models;
mod constants;

use clap::Parser;
use commands::Args;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    match args.entity_type {
        commands::Command::Encode(encode_cmd) => {
            encode_cmd.execute()?;
        }
        commands::Command::Decode(decode_cmd) => {
            decode_cmd.execute()?;
        }
    }
    Ok(())
}
