//! Transparente - Image background removal and conversion tool.
//!
//! This application provides both a GUI and CLI interface for processing images.

mod config;
mod generators;
mod gui;
mod lang;
mod cli;

use clap::Parser;
use anyhow::Result;

use crate::lang::LanguageManager;
use crate::generators::LogOutput;

#[derive(Parser, Debug)]
#[command(author, version, about = "Procesador de imágenes por lotes (Rust Edition)", long_about = None)]
struct Args {
    /// Carpeta con las imágenes originales
    #[arg(short, long)]
    input: Option<String>,

    /// Carpeta donde se guardarán los resultados
    #[arg(short, long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let lang = LanguageManager::default();
    let logger = LogOutput::StdOut;

    match (args.input, args.output) {
        (Some(input), Some(output)) => {
            cli::process_batch(&input, &output, &lang, &logger)?;
        }
        _ => {
            println!("{}", lang.t("log_gui_starting"));
            gui::run_gui()?;
        }
    }

    Ok(())
}
