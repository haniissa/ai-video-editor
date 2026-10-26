use clap::{Parser, Subcommand};

use crate::commands::analyze::analyze_video;
use crate::commands::export::run as export_video;
use crate::commands::export::run_edl as export_edl;
use crate::commands::export::save_edl as save_edl_file;

mod commands;
mod editor;
mod video;

#[derive(Parser)]
#[command(name = "ai-video-editor")]
#[command(about = "AI powered video editing engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands, // hold the chosen subcommand
}
#[derive(Subcommand)] // this generate the logic to trun enumvariants (analyze , edit ) into CLI subcommand option
enum Commands {
    Analyze {
        video: String,
    },
    Edit {
        video: String,
    },
    Export {
        input: String,
        output: String,
        cuts: Vec<String>,
    },
    ExportEdl {
        input: String,
        output: String,
        edl: String,
    },
    SaveEdl {
        edl: String,
        cuts: Vec<String>,
    },
}
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { video } => {
            // println!("video: {}", video);
            analyze_video(&video)?
        }
        Commands::Edit { video } => {
            println!("{}", video);
        }
        Commands::Export {
            input,
            output,
            cuts,
        } => {
            export_video(&input, &output, &cuts)?;
        }
        Commands::ExportEdl { input, output, edl } => export_edl(&input, &output, &edl)?,
        Commands::SaveEdl { edl, cuts } => {
            save_edl_file(&edl, &cuts)?;
        }
    }
    Ok(())
}
