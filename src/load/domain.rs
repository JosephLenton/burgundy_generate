use extern::toml::value::Table;

/// Yes, I really do have to do this.
/// It's to work around a Serde limition with default values.
/// They have to be a variable. Cannot be a value.
const TRUE: bool = true;

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub api_docs_url: Option<String>,
    pub name: String,
    pub header_user_agent: Option<bool>,
    pub url: String,
    pub query: Option<Table>,
    pub header: Option<Table>,
    pub types_mod: Option<String>,
    pub extern_crates: Option<Vec<String>>,
    pub lib_modules: Option<Vec<String>>,
}
