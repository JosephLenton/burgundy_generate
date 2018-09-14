#[derive(Deserialize, Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Patch,
    Trace,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        use super::Method::*;

        match self {
            Get => "Get",
            Post => "Post",
            Put => "Put",
            Delete => "Delete",
            Head => "Head",
            Options => "Options",
            Connect => "Connect",
            Patch => "Patch",
            Trace => "Trace",
        }
    }
}
