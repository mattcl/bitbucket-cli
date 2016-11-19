use std::io::Read;
use hyper::Client;
use hyper::Url;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use rustc_serialize::json;

use error::{ErrorKind, Result};
use pull_request::PullRequest;


pub struct Bitbucket {
    client: Client,
    headers: Headers,
    base_url: Url,
}

impl Bitbucket {
    pub fn new(auth: String, base_url: String) -> Result<Bitbucket> {
        let url = Url::parse(base_url.as_str())?;
        let mut headers = Headers::new();
        headers.set(Authorization(format!("Basic {}", auth).to_owned()));
        headers.set(ContentType(Mime(TopLevel::Application,
                                     SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));
        Ok(Bitbucket {
            client: Client::new(),
            headers: headers,
            base_url: url,
        })
    }

    pub fn create_pull_request(&self, pull_request: &PullRequest, dry: bool, debug: bool) -> Result<Url> {
        let url = self.base_url.join("")?;
        let body = json::encode(pull_request)?;

        if debug {
            println!("{}", body);
        }

        if dry {
            return Err(ErrorKind::DryRun.into())
        }

        let mut res =
            self.client.post(url).headers(self.headers.clone()).body(body.as_str()).send()?;
        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;

        if res.status.is_success() {
            if debug {
                println!("{}", response_body);
            }
            let data = json::Json::from_str(response_body.as_str())?;
            get_self_url(&data)
        } else {
            Err(ErrorKind::RequestError(response_body).into())
        }

    }
}

fn get_self_url(data: &json::Json) -> Result<Url> {
    let link = data.find_path(&["links", "self"])
        .and_then(|obj| obj.find("href"))
        .and_then(|obj| obj.as_string());

    if let Some(link) = link {
        let url = Url::parse(link)?;
        Ok(url)
    } else {
        Err(ErrorKind::MissingSelfLink.into())
    }
}
