use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VersifyArgs{
    /// Path to the file directory
    #[arg(short, long)]
    pub path: String,
    /// The name of the domain you want to modify. This is a list of valid domains:
    /// SATK, Mashup, SSC, SSIV, SCE, HCS, ImageImport, ImageDiscovery, SciStream, Metastore.
    #[arg(short, long)]
    pub domain: String,
    /// The build number of the apps, e.g. 4.0.8.10268
    #[arg(short, long)]
    pub build_number: String,
    // Choose the output directory. By default it'll create a new directory called "output" in the current working directory
    // #[arg(short, long)]
    // output: String

}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Create a new pull request
    PR(Command)
}

#[derive(Debug, Args)]
pub struct Command {
    #[clap(subcommand)]
    pub command: PrSubcommand
}

#[derive(Debug, Subcommand)]
pub enum PrSubcommand {
    /// Create a pull request
    Create(CreatePR)
}

#[derive(Debug, Args)]
pub struct CreatePR {
    pub name: String
}