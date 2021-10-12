use dotenv::dotenv;
use hatena::args::{parse_args, Command};
use hatena::command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = parse_args();

    match args.command {
        Command::List => command::print_list().await?,
        Command::Submit(args) => command::submit(args).await?,
    }

    Ok(())
}
