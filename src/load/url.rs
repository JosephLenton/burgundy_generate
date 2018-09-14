use crate::load::Method;
use crate::load::UrlPath;
use extern::toml::value::Table;

#[derive(Deserialize, Debug)]
pub struct Url {
    pub path: UrlPath,
    pub method: Method,
    pub api_docs_url: Option<String>,
    pub returns: Option<String>,
    pub body: Option<String>,
    pub header: Option<Table>,
    pub query: Option<Table>,
}
