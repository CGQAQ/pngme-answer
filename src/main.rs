mod chunk;
mod chunk_type;
mod commands;
mod crc;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::{
    fs::{self, read},
    str::FromStr,
};

use chunk_type::ChunkType;
use clap::Parser;

fn main() -> Result<()> {
    let args = commands::Opts::parse();

    match args.command {
        crate::commands::SubCommand::Encode {
            file,
            chunk_type,
            message,
            output,
        } => {
            let content = read(&file).map_err(|e| format!("Failed to read file: {}", e))?;
            let mut png = png::Png::try_from(content.as_ref())?;
            let chunk = chunk::Chunk::new(
                ChunkType::from_str(chunk_type.as_ref())?,
                message.into_bytes(),
            );
            png.append_chunk(chunk);
            let output = output.unwrap_or_else(|| file);
            fs::write(output, png.as_bytes())?;
        }

        crate::commands::SubCommand::Decode { file, chunk_type } => {
            let content = read(&file).map_err(|e| format!("Failed to read file: {}", e))?;
            let png = png::Png::try_from(content.as_ref())?;
            let chunk = png.chunk_by_type(&chunk_type);

            let message = match chunk {
                Some(chunk) => chunk.data_as_string().unwrap(),
                None => String::from(""),
            };

            println!("{}", message);
        }

        crate::commands::SubCommand::Remove { file, chunk_type } => {
            let content = read(&file).map_err(|e| format!("Failed to read file: {}", e))?;

            let mut png = png::Png::try_from(content.as_ref())?;
            png.remove_chunk(&chunk_type)?;

            fs::write(file, png.as_bytes())?;
        }

        crate::commands::SubCommand::Print { file } => {
            let content = read(&file).map_err(|e| format!("Failed to read file: {}", e))?;
            let png = png::Png::try_from(content.as_ref())?;
            println!("{}", png)
        }
    }

    Ok(())
}
