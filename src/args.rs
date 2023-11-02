use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VersifyArgs {
    #[clap(subcommand, name = "github authentication")]
    pub command: Option<EntityType>,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create a new branch
    CreateBranch(CreateBranchCommand),
    /// Delete a target branch
    DeleteBranch(DeleteBranchCommand),
    /// Update the packages.txt in the repository
    UpdateBranch(UpdateBranchCommand),
    /// Create a pull request
    CreatePr(CreatePrCommand),
    /// Download the packages.txt file from the repository
    Download(DownloadCommand),
    /// Update the packages.txt file from the repository
    Update(UpdateCommand),
    /// List all branches of the repository
    List(ListCommand),
    /// List all the currently open pull requests
    ListPr(ListPrCommand),
    /// Close current open pull request. You should use the pull request number for this matter.
    ClosePr(ClosePrCommand),
}


#[derive(Debug, Args)]
pub struct CreateBranchCommand {
    pub source: String,
    pub new_branch: String,
}

#[derive(Debug, Args)]
pub struct UpdateBranchCommand {
    /// Commit message
    #[arg(short, long)]
    pub message: String,
    /// Target branch to be updated
    #[arg(short, long)]
    pub target_branch: String,
    /// File path
    #[arg(short, long)]
    pub path: Option<String>,
    /// The name of the domain you want to modify. This is a list of valid domains:
    /// SATK, Mashup, SSC, SSIV, SCE, HCS, ImageImport, ImageDiscovery, SciStream, Metastore.
    #[arg(short, long)]
    pub domain: Option<String>,
    /// The build number of the apps, e.g. 3.0.8.10268
    #[arg(short, long)]
    pub version: Option<String>,

}

#[derive(Debug, Args)]
pub struct CreatePrCommand {
    /// Pull request title
    #[arg(short, long)]
    pub title: String,
    /// Pull request body message
    #[arg(short, long)]
    pub message: String,
    /// Source branch (i.e. your current working branch)
    #[arg(long)]
    pub branch: String,
    /// Target branch (i.e. branch to merge your pull request)
    #[arg(long)]
    pub target_branch: String,
}

#[derive(Debug, Args)]
pub struct DeleteBranchCommand {
    /// Branch to be deleted (the branch you have created)
    pub branch: String,
}

#[derive(Debug, Args)]
pub struct DownloadCommand {
    /// Choose the branch in the repository. By default, it'll pull the packages.txt from main
    #[arg(short, long)]
    pub branch: String,
    /// Choose the output directory. By default, it'll create a new directory called "tmp" in the current working directory
    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output: Option<String>,
}

#[derive(Debug, Args)]
pub struct ListCommand {}

#[derive(Debug, Args)]
pub struct ListPrCommand {}

#[derive(Debug, Args)]
pub struct ClosePrCommand {
    pub pr_number: String
}

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