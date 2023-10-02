use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VersifyArgs{
    /// Path to the file directory
    pub path: String,

    /// The name of the domain you want to modify like, for example, SDK or InVivo, etc.
    pub domain: String,

    /// The build number of the apps
    pub build_number: String
}