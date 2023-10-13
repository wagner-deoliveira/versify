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
    /// Choose the output directory. By default, it'll create a new directory called "output" in the current working directory
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output: Option<String>,
    //// Command to create a new pull request
    // #[clap(subcommand, name = "GITHUB_ACTIONS")]
    // pub github_actions: Option<EntityType>,
}

//TODO: Add support to choose the output directory and subcommand to create a pull request using GitHub API

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct GitHubActions {
    #[clap(subcommand, name = "create_pull_request")]
    pub entity_type: EntityType,
}
#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a pull request
    Create(PrCommand),
    /// Download the packages.txt file from the repository
    Download(DownloadCommand)
}

#[derive(Debug, Args)]
pub struct PrCommand {
    #[clap(subcommand, name = "create pull request")]
    pub command: PrSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum PrSubCommand {
    /// Create a pull request
    Create(PullRequestCommand),
}

#[derive(Debug, Args)]
pub struct PullRequestCommand {
    /// Create a pull request with a specific title
    pub title: String,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    #[clap(subcommand, name = "download file")]
    pub command: DownloadSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum DownloadSubCommand {
    /// Create a pull request
    Download(DownloadRequestCommand),
}

#[derive(Debug, Args)]
pub struct DownloadRequestCommand {
    /// Create a pull request with a specific title
    pub name: String,
}
