use std::collections::HashMap;
use std::io::Read;

use hyper::Client;
use hyper::Url;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;
use rustc_serialize::json;

use error::{Error, ErrorKind, Result};
use pull_request::PullRequest;

pub struct UserSearchResult {
    users: HashMap<String, String>,
}

impl UserSearchResult {
    pub fn print_tty(&self, force_colorize: bool) {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .padding(1, 1)
            .separator(format::LinePosition::Title,
                       format::LineSeparator::new('-', '-', '-', '-'))
            .build();

        table.set_format(format);
        table.set_titles(Row::new(vec![Cell::new("name"), Cell::new("slug")]));

        for (user, slug) in &self.users {
            table.add_row(Row::new(vec![Cell::new(user), Cell::new(slug)]));
        }

        table.print_tty(force_colorize);
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }
}

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

    pub fn create_pull_request(&self,
                               pull_request: &PullRequest,
                               dry: bool,
                               debug: bool)
                               -> Result<Url> {

        let component = format!("rest/api/1.0/projects/{}/repos/{}/pull-requests",
                                pull_request.project()
                                    .ok_or::<Error>(ErrorKind::InvalidPullRequest("Missing toRef"
                                            .to_string())
                                        .into())?,
                                pull_request.slug()
                                    .ok_or::<Error>(ErrorKind::InvalidPullRequest("Missing toRef"
                                            .to_string())
                                        .into())?);
        let url = self.base_url.join(&component)?;
        let body = json::encode(pull_request)?;

        if debug {
            println!("{}", body);
        }

        if dry {
            return Err(ErrorKind::DryRun.into());
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

    pub fn user(&self, filter: &str, debug: bool) -> Result<UserSearchResult> {
        let mut url = self.base_url.join("rest/api/1.0/users")?;
        url.query_pairs_mut().append_pair("filter", filter);

        if debug {
            println!("{}", url);
        }

        let mut res = self.client.get(url).headers(self.headers.clone()).send()?;

        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;
        if res.status.is_success() {
            if debug {
                println!("{}", response_body);
            }
            let data = json::Json::from_str(response_body.as_str())?;
            get_user_search_result(&data)
        } else {
            Err(ErrorKind::RequestError(response_body).into())
        }
    }
}

fn get_self_url(data: &json::Json) -> Result<Url> {
    let link = data.find_path(&["links", "self"])
        .and_then(|obj| obj.as_array())
        .and_then(|obj| obj.iter().next())
        .and_then(|obj| obj.find("href"))
        .and_then(|obj| obj.as_string());

    if let Some(link) = link {
        let url = Url::parse(link)?;
        Ok(url)
    } else {
        Err(ErrorKind::MissingSelfLink.into())
    }
}

fn get_user_search_result(data: &json::Json) -> Result<UserSearchResult> {
    if let Some(results) = data.find("values").and_then(|obj| obj.as_array()) {
        let mut users = HashMap::new();
        for result in results {
            let slug = result.find("slug").and_then(|obj| obj.as_string()).unwrap_or("missing slug");
            let name = result.find("displayName").and_then(|obj| obj.as_string()).unwrap_or("missing displayName");

            users.insert(name.to_string(), slug.to_string());
        }

        return Ok(UserSearchResult {
            users: users
        })
    }

    Err(ErrorKind::InvalidResponse.into())
}
