extern crate serde;
extern crate serde_derive;
use serde::Deserialize;
extern crate reqwest;

// # use reqwest::Error;
use std::env;
// # use std::path::PathBuf;
use structopt::StructOpt;

// Basically we're just doing this:
// curl -X GET -k -H 'Authorization: token abcdef1234567890abcdef' \
// -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' \
// 'https://app.fossa.com/api/organizations?$title=uber' | \
// jq -r '.[]|[.id, .title, .access_level, .created, .contributors_updated]|@csv' | \
// sort -k1n
#[derive(Deserialize, Debug)]
struct Org {
    id: u32,
    title: String,
    access_level: String,
    created: String,
    contributors_updated: String,
}

fn get_orgs(baseurl: &str, api_token: &str,
            orgname: &str, debug: bool) -> Result<(), reqwest::Error> {
    let client =  reqwest::Client::new();
    let token_string = format!("token {}", api_token);

    let url = format!("{}api/organizations", baseurl);
    let mut response = client.get(&url)
        .header("Authorization", &token_string )
        .query(&[("title", orgname)])
        .send()?;
    if debug {
        println!("{}", response.status());
    }
    let orgs: Vec<Org> = response.json()?;
    // # println!("{:?}", orgs);
    for o in orgs.iter() {
        println!("{:>col1$} {:col2$} {} {} {}", o.id , o.title, o.access_level,
                 o.created, o.contributors_updated, col1=6, col2=30);
    }
    Ok(())

}

///
/// Make Admin Fast!!1!ONE
/// Specifically admin tasks like looking up org ids and switching orgs
#[derive(Debug, StructOpt)]
struct CliOpts {
    /// Debug issues with fossa admin tool
    #[structopt(short, long)]
    debug: bool,

    /// org name
    #[structopt(short, long, default_value = "Macrosoft")]
    orgname: String,
}

fn main() {
    let opts = CliOpts::from_args();
    let baseurl = "https://app.fossa.com/";
    let api_token = env::var("FOSSA_API_KEY")
        .expect("FOSSA_API_KEY not found");
    get_orgs(&baseurl, &api_token, &opts.orgname, opts.debug)
        .expect("OOPS: problem getting orgs");
}
