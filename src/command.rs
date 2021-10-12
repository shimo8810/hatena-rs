use crate::args::Submit;
use atom_syndication::Feed;
use reqwest;
use std::env;
use std::process;
use url::Url;

const USER: &str = "HATENA_USER";
const APIKEY: &str = "HATENA_APIKEY";
const ENDPOINT: &str = "HATENA_ENDPOINT";

fn read_envvar() -> Result<(String, String, String), env::VarError> {
    let user = env::var(USER)?;
    let apikey = env::var(APIKEY)?;
    let endpint = env::var(ENDPOINT)?;

    Ok((user, apikey, endpint))
}

pub async fn submit(args: Submit) -> Result<(), Box<dyn std::error::Error>> {
    println!("submit {:?}", args);
    Ok(())
}

pub async fn print_list() -> Result<(), Box<dyn std::error::Error>> {
    let (username, password, endpoint) = match read_envvar() {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut opt_url = Url::parse(&endpoint)
        .ok()
        .map(|x| x.join("atom/entry").unwrap());
    let mut entries = Vec::new();

    while let Some(url) = opt_url {
        let resp = reqwest::Client::new()
            .get(url)
            .basic_auth(&username, Some(&password))
            .send()
            .await?
            .text()
            .await?;

        let mut feed: Feed = resp.parse().unwrap();
        entries.append(&mut feed.entries);
        opt_url = feed
            .links
            .iter()
            .find(|&link| link.rel == "next")
            .map(|link| Url::parse(&link.href).unwrap());
    }

    println!("Entries:");
    for e in entries.iter() {
        println!("    {}", e.title.value);
    }
    Ok(())
}
