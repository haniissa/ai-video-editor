use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-video-editor")]
#[command(about = "AI powered video editing engine")]

struct Cli {
    #[command(subcommand)]
    command: Commands,// hold the chosen subcommand
}
#[derive(Subcommand)] // this generate the logic to trun enumvariants (analyze , edit ) into CLI subcommand option
enum Commands {
    Analyze {
        video: String,
    },
    Edit{
        video:String,
    },
}
fn main() {
    let cli = Cli::parse();

    match cli.command{
        Commands::Analyze { video } => {
            println!("Analyzing video:");
            println!("{}", video);
        }

        Commands::Edit { video } => {
            println!("Editing video:");
            println!("{}", video);
        }
    }
}
