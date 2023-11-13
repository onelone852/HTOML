use toml::{Table, Value};

use crate::arg::Argument;

use super::{
    error::{HtomlError, Result},
    html::Html,
};

pub struct Htoml {
    toml: Table,
    html: Html,
}

static VOID_ELEMENT: [&str; 2] = ["br", "hr"];
static SIMPLE_REAL_ELEMENT: [&str; 8] = ["p", "b", "i", "strong", "mark", "u", "s", "small"];

impl Htoml {
    pub fn new(toml: String) -> Result<Self> {
        Ok(Self {
            toml: toml
                .parse::<Table>()
                .map_err(|e| HtomlError::InvalidToml(e))?,
            html: Html::default(),
        })
    }

    fn version(&mut self) -> Result<()> {
        if let Some(Value::String(ver)) = self.toml.get("html") {
            self.html.insert_void_elem(&format!("!DOCTYPE {}", ver));
            Ok(())
        } else {
            Err(HtomlError::UndeclaredFile)
        }
    }

    fn parse_head(&mut self) -> Result<()> {
        let raw_head = self.toml.get("head");
        self.html.open_elem("head");
        if let Some(Value::Table(head)) = raw_head {
            for (key, val) in head {
                match key.as_str() {
                    "title" => self.html.insert_elem("title", val.as_str().unwrap()),
                    _ => return Err(HtomlError::UnknownHead(key.to_string())),
                };
            }
        } else if let Some(_) = raw_head {
            return Err(HtomlError::NonTableHead);
        }
        self.html.close_elem("head");
        Ok(())
    }

    fn parse_a_element(html: &mut Html, a: &Table, elem_cont: &Value) -> Result<()> {
        let href = a
            .get("href")
            .and_then(|val| val.as_str())
            .ok_or(HtomlError::AWithoutHref)?;
        html.open_elem_with_args(
            "a",
            &[Argument {
                name: "href",
                val: href,
            }],
        );
        Self::parse_element(html, elem_cont)?;
        html.close_elem("a");
        Ok(())
    }

    fn parse_element(html: &mut Html, elem: &Value) -> Result<()> {
        if let Value::String(s) = elem {
            html.push(s);
        } else if let Value::Table(table) = elem {
            let elem_cont = table.get("cont").ok_or(HtomlError::NoContent);
            let elem_type = table
                .get("type")
                .ok_or(HtomlError::UntypedElement)?
                .as_str()
                .ok_or(HtomlError::UntypedElement)?;
            match elem_type {
                real if SIMPLE_REAL_ELEMENT.contains(&real) => {
                    html.open_elem(real);
                    Self::parse_element(html, elem_cont?)?;
                    html.close_elem(real);
                }
                void if VOID_ELEMENT.contains(&void) => html.insert_void_elem(void),
                "a" => Self::parse_a_element(html, table, elem_cont?)?,
                _ => return Err(HtomlError::UnknownElement(elem_type.to_string())),
            };
        } else if let Value::Array(arr) = elem {
            for val in arr {
                Self::parse_element(html, val)?;
            }
        } else {
            return Err(HtomlError::UnknownContent);
        }
        Ok(())
    }

    fn parse_body(&mut self) -> Result<()> {
        let raw_body = self.toml.get("body");
        let lang = self.toml.get("lang");
        let mut args: Vec<Argument> = Vec::with_capacity(1);
        if let Some(Value::String(s)) = lang {
            args.push(Argument {
                name: "lang",
                val: s,
            });
        } else if let Some(_) = lang {
            return Err(HtomlError::NonStrLang);
        }
        self.html.open_elem_with_args("body", &args);
        if let Some(body) = raw_body {
            Self::parse_element(&mut self.html, body)?;
        }
        self.html.close_elem("body");
        Ok(())
    }

    fn parse_html(&mut self) -> Result<()> {
        self.html.open_elem("html");
        self.parse_head()?;
        self.parse_body()?;
        self.html.close_elem("html");
        Ok(())
    }

    pub fn parse(mut self) -> Result<String> {
        self.version()?;
        self.parse_html()?;
        Ok(self.html.into())
    }
}
