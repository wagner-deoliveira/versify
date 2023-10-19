use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VersifyArgs {
    #[clap(subcommand, name = "github authentication")]
    pub command: Option<EntityType>,
}

//TODO: Add support to choose the output directory and subcommand to create a pull request using GitHub API

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a pull request
    Create(PrCommand),
    /// Download the packages.txt file from the repository
    Download(DownloadCommand),
    /// Update the packages.txt file from the repository
    Update(UpdateCommand),
}


#[derive(Debug, Args)]
pub struct PrCommand {
    pub string: Option<String>,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    pub string: Option<String>,
}


#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// Path to the file directory
    #[arg(short, long)]
    pub path: String,
    /// The name of the domain you want to modify. This is a list of valid domains:
    /// SATK, Mashup, SSC, SSIV, SCE, HCS, ImageImport, ImageDiscovery, SciStream, Metastore.
    #[arg(short, long)]
    pub domain: String,
    /// The build number of the apps, e.g. 3.0.8.10268
    #[arg(short, long)]
    pub build_number: String,
    /// Choose the output directory. By default, it'll create a new directory called "output" in the current working directory
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output: Option<String>,
}