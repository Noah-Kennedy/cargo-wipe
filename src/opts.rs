use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Command {
    /// Recursively finds and optionally wipes all <target> or <node_modules> folders that are found in the current path. Add `-w` to wipe all folders found. USE WITH CAUTION!
    Wipe(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub folder_name: FolderNameEnum,
}

#[derive(Debug, StructOpt)]
pub enum FolderNameEnum {
    /// Recursively finds and optionally wipes all <node_modules> folders that are found in the current path
    #[structopt(name = "node_modules")]
    NodeModules(Opts),
    /// Alias to node_modules
    Node(Opts),
    /// Recursively finds and optionally wipes all <target> folders that are found in the current path
    Target(Opts),
    /// Alias to target
    Rust(Opts),
    /// Looks for cmake-build-debug
    #[structopt(name = "cmake")]
    CMake(Opts),
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// CAUTION! If set will wipe all found folders! Unset by default
    #[structopt(short, long)]
    pub wipe: bool,
}
