#[derive(Debug, PartialEq, Eq, Hash)]
pub enum UrlPathPart {
    StringPart { path: String },
    VariablePart { name: String, rust_type: String },
}

impl UrlPathPart {
    pub fn as_name<'a>(&'a self) -> &'a str {
        match self {
            UrlPathPart::StringPart { path } => &path,
            UrlPathPart::VariablePart { name, .. } => &name,
        }
    }
}
