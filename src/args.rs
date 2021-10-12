use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Submit {
    pub filename: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Submits an article to hatena blog
    #[structopt(name = "submit")]
    Submit(Submit),
    /// Prints list of articles
    #[structopt(name = "list")]
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "hatena")]

pub struct Args {
    #[structopt(subcommand)]
    pub command: Command,
}

pub fn parse_args() -> Args {
    Args::from_args()
}
