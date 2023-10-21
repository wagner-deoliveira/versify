use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VersifyArgs {
    #[clap(subcommand, name = "github authentication")]
    pub command: Option<EntityType>,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a pull request
    CreatePR(PrCommand),
    /// Download the packages.txt file from the repository
    Download(DownloadCommand),
    /// Update the packages.txt file from the repository
    Update(UpdateCommand),
    /// List all branches of the repository
    List(ListCommand),
}


#[derive(Debug, Args)]
pub struct PrCommand {
    pub source: String,
    pub new_branch: String,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    pub string: Option<String>,
}

#[derive(Debug, Args)]
pub struct ListCommand {}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// Path to the file directory
    #[arg(short, long)]
    pub path: Option<String>,
    /// The name of the domain you want to modify. This is a list of valid domains:
    /// SATK, Mashup, SSC, SSIV, SCE, HCS, ImageImport, ImageDiscovery, SciStream, Metastore.
    #[arg(short, long)]
    pub domain: String,
    /// The build number of the apps, e.g. 3.0.8.10268
    #[arg(short, long)]
    pub version: String,
    /// Choose the output directory. By default, it'll create a new directory called "output" in the current working directory
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output: Option<String>,
    /// Choose the branch in the repository. By default, it'll pull the packages.txt from main
    #[arg(short, long)]
    pub branch: Option<String>,
}