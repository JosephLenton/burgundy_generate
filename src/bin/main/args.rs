use structopt::StructOpt;

/// Command line arguments.
///
/// See structopt for how this works https://github.com/TeXitoi/structopt
///
#[allow(missing_docs)]
#[derive(StructOpt, Debug)]
#[structopt(
    name = "Burgundy Generate",
    about = "For generating an API from a TOML file",
    author = "",
)]
crate struct Args {
    #[structopt(
        long = "out",
        help = "The directory where this is outputted to"
    )]
    crate directory: Option<String>,

    #[structopt(help = "The file to generate the API from")]
    crate api_config_file: String,
}

/// Builds a new args from command line arguments provided when this
/// started.
///
crate fn from_cmd_args() -> Args {
    Args::from_args()
}
