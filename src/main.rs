extern crate serde;
extern crate serde_derive;
use serde::Deserialize;
use serde::de::Deserializer;
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
    #[serde(deserialize_with="parse_u32")]
    contributor_count: u32,
    #[serde(deserialize_with="parse_string")]
    contributors_updated: String,
}


// Turn nulls into proper types in the input which gives us nice looking data
// Some Deserialiazer magic wrapping a map function wrapping unwrap_or (etc)
// Stolen from here (and reformatted for clarity):
// https://stackoverflow.com/questions/44205435/how-to-deserialize-a-json-file-which-contains-null-values-using-serde
// TODO There is proably a way to parameterize the type and make this more portable &
// eliminate duplication.
fn parse_string<'de, D>(d: D) -> Result<String, D::Error>
where 
    D: Deserializer<'de>, {
        Deserialize::deserialize(d)
            .map(|x: Option<_>| x.unwrap_or("none".to_string()))
}
fn parse_u32<'de, D>(d: D) -> Result<u32, D::Error>
where 
    D: Deserializer<'de>, {
        Deserialize::deserialize(d)
            .map(|x: Option<_>| x.unwrap_or(0 as u32))
}


fn get_orgs(baseurl: &str,
            api_token: &str,
            orgname: &str,
            debug: bool) -> Result<Vec<Org>, reqwest::Error> {
    let client =  reqwest::Client::new();
    let token_string = format!("token {}", api_token);

    let url = format!("{}api/organizations?$title={}", baseurl, orgname);
    let mut response = client.get(&url)
        .header("Authorization", &token_string )
        .send()?;
    if debug {
        println!("URL Sent: {}", url);
        println!("HTTP Response: {}", response.status());
    }
    let orgs: Vec<Org> = response.json()?;
    Ok(orgs)
}

fn set_org(baseurl: &str,
           api_token: &str,
           orgname: &str,
           debug: bool) -> Result<Vec<Org>, reqwest::Error> {
    // curl 'https://app.fossa.com/api/users/14602' \
    // -X PUT \
    // --data-raw 'organizationId=11665'
    let client =  reqwest::Client::new();
    let token_string = format!("token {}", api_token);

    let url = format!("{}api/users/{}", baseurl, userid);
    let mut response = client.get(&url)
        .header("Authorization", &token_string )
        .send()?;
    if debug {
        println!("URL Sent: {}", url);
        println!("HTTP Response: {}", response.status());
    }
    let orgs: Vec<Org> = response.json()?;
    Ok(orgs)
}

fn format_org(org: &Org) -> String {
    format!(
        "{:>col1$}  {:col2$}  {:col3$}  {:col4$}  {}  {}\n",
        org.id.to_string(),
        org.title,
        org.access_level,
        org.contributor_count.to_string(),
        org.created,
        org.contributors_updated,
        col1 = 6,
        col2 = 36,
        col3 = 10,
        col4 = 4
    )
}

///
/// Make Admin Fast!!1!ONE
/// Specifically admin tasks like looking up org ids and switching orgs
#[derive(Debug, StructOpt)]
struct CliOpts {
    /// Debug issues with fossa admin tool
    #[structopt(short, long)]
    debug: bool,

    /// Organization name for search
    #[structopt(short, long, conflicts_with = "setid")]
    orgname: Option<String>,

    /// Organization ID to set
    #[structopt(short, long, conflicts_with = "orgname")]
    setid: Option<u32>
}

fn main() {
    let opts = CliOpts::from_args();
    let baseurl = "https://app.fossa.com/";
    let api_token = env::var("FOSSA_API_KEY")
        .expect("FOSSA_API_KEY environment variable not found");
    match (opts.orgname, opts.setid) {
        (Some(orgname), _) => {
            let orgs = get_orgs(&baseurl, &api_token, &orgname, opts.debug)
                .expect("OOPS: problem getting orgs");
            let mut table: String = "".to_string();
            for org in orgs.iter() {
                table.push_str(&format_org(org));
            };
            println!("{}", table)
        },
        (_, Some(setid)) => {
            println!("{:?}", setid)
        },
        (_, _) => {
            println!("Unsupported configuration: check help")
        },
    };
}
