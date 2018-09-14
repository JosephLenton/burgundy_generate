use crate::load::Api;
use crate::load::Method;
use crate::load::Url;
use crate::load::UrlPathPart;
use crate::output_dest_writer::OutputDestWriter;
use extern::inflector::Inflector;
use extern::toml::value::Value;
use std::collections::HashMap;
use std::io::Write;

pub fn print(api: Api, writer: &mut OutputDestWriter) {
    let urls_map = get_urls_by_method(&api);

    print_domain(&api, &urls_map, writer);
    print_end_point_clients(&api, &urls_map, writer);
}

fn get_urls_by_method<'a>(api: &'a Api) -> HashMap<Method, UrlPathNode<'a>> {
    let mut urls_by_method: HashMap<Method, UrlPathNode<'a>> = HashMap::new();

    for url in api.url.iter() {
        let maybe_url_node = urls_by_method.get_mut(&url.method);

        if let Some(url_node) = maybe_url_node {
            url_node.walk_url(&url, &url.path.parts);
        } else {
            let mut url_node = UrlPathNode::new();
            url_node.walk_url(&url, &url.path.parts);
            urls_by_method.insert(url.method, url_node);
        }
    }

    urls_by_method
}

struct UrlPathNode<'a> {
    mappings: HashMap<&'a UrlPathPart, UrlPathNode<'a>>,
    is_end_node: Option<&'a Url>,
}

impl<'a> UrlPathNode<'a> {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            is_end_node: None,
        }
    }

    fn walk_url(&mut self, url: &'a Url, parts: &'a [UrlPathPart]) {
        if parts.len() == 0 {
            self.is_end_node = Some(url);
        } else {
            let head = &parts[0];
            let maybe_next = self.mappings.get_mut(&head);

            if let Some(next) = maybe_next {
                next.walk_url(url, &parts[1..]);
            } else {
                let mut next = UrlPathNode::new();
                next.walk_url(url, &parts[1..]);
                self.mappings.insert(&head, next);
            }
        }
    }
}

fn print_domain<'a>(
    api: &Api,
    urls: &HashMap<Method, UrlPathNode<'a>>,
    writer: &mut OutputDestWriter,
) {
    let domain = &api.domain;
    let header_user_agent = domain.header_user_agent.unwrap_or(true);
    let mut out = writer.get_writer(&[&"lib"]);

    writeln!(out, "extern crate burgundy;");
    writeln!(out, "extern crate serde;");
    writeln!(out, "#[macro_use]");
    writeln!(out, "extern crate serde_derive;");
    domain.extra_crates.as_ref().and_then(|extra_crates| {
        for extra_crate in extra_crates {
            writeln!(out, "extern crate {};", extra_crate);
        }

        Some(())
    });

    for key in urls.keys() {
        let method_lower = key.as_str().to_lowercase();

        writeln!(out, "mod {};", method_lower);
        writeln!(
            out,
            "pub use self::{}::{}{};",
            method_lower,
            domain.name,
            key.as_str()
        );
    }

    writeln!(out, "");

    if let Some(api_docs_url) = &domain.api_docs_url {
        writeln!(out, "/// See {}", api_docs_url);
    }

    writeln!(
        out,
        r###"pub struct {domain_name} {{
  domain: burgundy::Domain,
}}
  "###,
        domain_name = domain.name
    );

    writeln!(out, "impl {domain_name} {{", domain_name = domain.name);
    writeln!(out, "  pub fn new(");
    if let Some(headers) = &domain.header {
        for (name, rust_type_value) in headers {
            if let Value::String(rust_type) = rust_type_value {
                writeln!(out, "    {} : {},", name.to_snake_case(), rust_type);
            } else {
                panic!("Unknown structure given for parameter {}", name)
            }
        }
    }

    if let Some(queries) = &domain.query {
        for (name, rust_type_value) in queries {
            if let Value::String(rust_type) = rust_type_value {
                writeln!(out, "    {} : {},", name.to_snake_case(), rust_type);
            } else {
                panic!("Unknown structure given for parameter {}", name)
            }
        }
    }
    writeln!(
        out,
        r###"  ) -> Self {{
    let mut domain = burgundy::Domain::new(&"{domain_url}");"###,
        domain_url = domain.url
    );

    if header_user_agent {
        writeln!(
            out,
            r###"
    let user_agent = format!("{domain_name} (Rust, Burgundy)/{{}}", env!("CARGO_PKG_VERSION"));
    domain.header(&"User-Agent", &user_agent.as_str());"###,
            domain_name = domain.name
        );
    }

    if let Some(headers) = &domain.header {
        for (name, _) in headers {
            writeln!(
                out,
                r###"    domain.header(&"{name}", &{name});"###,
                name = name
            );
        }
    }

    if let Some(queries) = &domain.query {
        for (name, _) in queries {
            writeln!(
                out,
                r###"    domain.query(&"{name}", &{name});"###,
                name = name
            );
        }
    }

    writeln!(
        out,
        r###"
    Self {{ domain }}
  }}"###
    );

    for key in urls.keys() {
        let key_name = key.as_str().to_lowercase();

        writeln!(
            out,
            r###"
  pub fn {method_lower}(&self) -> self::{method_lower}::{domain_name}{method} {{
    self::{method_lower}::{domain_name}{method} {{
      path: self.domain.{method_lower}(),
    }}
  }}"###,
            method_lower = key_name,
            domain_name = domain.name,
            method = key.as_str()
        );
    }

    writeln!(out, "}}");
}

fn print_end_point_clients<'a>(
    api: &Api,
    urls: &HashMap<Method, UrlPathNode<'a>>,
    writer: &mut OutputDestWriter,
) {
    for (method, url_node) in urls.iter() {
        let type_name = format!("{}_{}", api.domain.name, method.as_str()).to_class_case();
        let mod_name = method.as_str().to_snake_case();
        let path_parts: Vec<&str> = vec![&mod_name];

        print_end_point_client(api, &type_name, &mod_name, path_parts, &url_node, writer);
    }
}

fn print_end_point_client<'a>(
    api: &Api,
    type_name: &'a str,
    mod_name: &'a str,
    path_parts: Vec<&'a str>,
    url: &UrlPathNode<'a>,
    writer: &mut OutputDestWriter,
) {
    let mut out = writer.get_writer(&path_parts);
    let types_mod = api
        .domain
        .types_mod
        .as_ref()
        .map(String::as_str)
        .unwrap_or(&"");

    for (key, _) in url.mappings.iter() {
        let key_name = key.as_name().to_snake_case();
        let next_type_name = format!("{}_{}", type_name, key.as_name()).to_class_case();

        writeln!(out, "mod {};", key_name);
        writeln!(out, "pub use self::{}::{};", key_name, next_type_name);
    }

    writeln!(
        out,
        r###"
pub struct {type_name} {{
  pub(crate) path: burgundy::Path,
}}
"###,
        type_name = type_name,
    );

    write!(out, "impl {type_name} {{", type_name = type_name,);

    for key in url.mappings.keys() {
        match key {
            UrlPathPart::StringPart { ref path } => {
                let name_as_snake = path.to_snake_case();
                let next_type_name = format!("{}_{}", type_name, path).to_class_case();

                writeln!(
                    out,
                    r###"
  pub fn {mod_name}(self) -> self::{mod_name}::{type_name} {{
    self::{mod_name}::{type_name} {{
      path: self.path.push(&"{path}"),
    }}
  }}"###,
                    mod_name = name_as_snake,
                    type_name = next_type_name,
                    path = path
                );
            }
            UrlPathPart::VariablePart {
                ref name,
                ref rust_type,
            } => {
                let name_as_snake = name.to_snake_case();
                let next_type_name = format!("{}_{}", type_name, name).to_class_case();

                writeln!(
                    out,
                    r###"
  pub fn {name}(self, {name} : {rust_type}) -> self::{mod_name}::{type_name} {{
    self::{mod_name}::{type_name} {{
      path: self.path.push(&{name}),
    }}
  }}"###,
                    mod_name = name_as_snake,
                    type_name = next_type_name,
                    rust_type = rust_type,
                    name = name_as_snake,
                );
            }
        }
    }

    if let Some(url) = url.is_end_node {
        writeln!(out, "");
        if let Some(api_docs_url) = &url.api_docs_url {
            writeln!(out, "  /// See {}", api_docs_url);
        }

        writeln!(
            out,
            r###"  pub fn run(self) -> burgundy::Result<{types_mod}{returns}> {{
    self.path"###,
            types_mod = types_mod,
            returns = url.returns.as_ref().map(String::as_str).unwrap_or(&"()")
        );

        if let Some(headers) = &url.header {
            for (param_name, param_value) in headers {
                writeln!(
                    out,
                    r#"        .header(&"{param_name}", &"{param_value}")"#,
                    param_name = param_name,
                    param_value = param_value
                );
            }
        }

        if let Some(queries) = &url.query {
            for (param_name, param_value) in queries {
                writeln!(
                    out,
                    r#"        .query(&"{param_name}", &"{param_value}")"#,
                    param_name = param_name,
                    param_value = param_value
                );
            }
        }

        writeln!(
            out,
            r#"        .execute_as_json::<{types_mod}{returns}>()
  }}"#,
            types_mod = types_mod,
            returns = url.returns.as_ref().map(String::as_str).unwrap_or(&"()")
        );
    }

    writeln!(out, "}}");

    for (key, next_url_node) in url.mappings.iter() {
        let key_name = key.as_name().to_snake_case();
        let next_type_name = format!("{}_{}", type_name, key.as_name()).to_class_case();
        let next_mod_name = format!("{}::{}", mod_name, key_name);
        let mut next_path_parts = path_parts.clone();
        next_path_parts.push(&key_name);

        print_end_point_client(
            api,
            &next_type_name,
            &next_mod_name,
            next_path_parts,
            next_url_node,
            writer,
        );
    }
}
