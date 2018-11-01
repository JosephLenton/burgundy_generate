use crate::load::UrlPathPart;
use extern::serde::de::Deserialize;
use extern::serde::de::Deserializer;
use extern::serde::de::Error;
use extern::serde::de::Visitor;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

static URL_VARIABLE_REGEX: &'static str =
    r###"^<([a-zA-Z_][a-zA-Z_0-9]+):([a-zA-Z_:][a-zA-Z_0-9:]+)>$"###;

#[derive(Debug)]
pub struct UrlPath {
    pub parts: Vec<UrlPathPart>,
}

impl<'de> Deserialize<'de> for UrlPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(deserializer.deserialize_str(UrlVisitor)?)
    }
}

struct UrlVisitor;

impl<'de> Visitor<'de> for UrlVisitor {
    type Value = UrlPath;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("A description of an url")
    }

    fn visit_str<E>(self, text: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let variable_regex = regex::RegexBuilder::new(&URL_VARIABLE_REGEX)
            .case_insensitive(true)
            .build()
            .unwrap();

        let parts = text
            .split("/")
            .map(|txt| {
                if let Some(captures) = variable_regex.captures(txt) {
                    let name = captures.get(1).unwrap().as_str().to_string();
                    let rust_type = captures.get(2).unwrap().as_str().to_string();

                    UrlPathPart::VariablePart { name, rust_type }
                } else {
                    UrlPathPart::StringPart {
                        path: txt.to_string(),
                    }
                }
            })
            .collect();

        Ok(UrlPath { parts })
    }
}
