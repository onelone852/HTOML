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

    fn parse_a_element<'a>(attrs: &mut Vec<Argument<'a>>, a: &'a Table) -> Result<()> {
        let href = a
            .get("href")
            .and_then(|val| val.as_str())
            .ok_or(HtomlError::AWithoutHref)?;
        attrs.push(Argument {
            name: "href",
            val: href,
        });

        Ok(())
    }

    fn parse_class_and_id<'a>(
        attrs: &mut Vec<Argument<'a>>,
        elem: &'a Table,
        class_str: &'a mut String,
    ) -> Result<()> {
        let classes = elem.get("class");
        let id = elem.get("id");

        if let Some(Value::String(s)) = classes {
            attrs.push(Argument {
                name: "class",
                val: s,
            });
        } else if let Some(Value::Array(arr)) = classes {
            for raw_class in arr {
                let class = raw_class.as_str().ok_or(HtomlError::UnknownClass)?;
                class_str.push(' ');
                class_str.push_str(class);
            }
            attrs.push(Argument {
                name: "class",
                val: class_str.trim_start(),
            })
        } else if let Some(_) = classes {
            return Err(HtomlError::UnknownClass);
        }

        if let Some(Value::String(id_str)) = id {
            attrs.push(Argument {
                name: "id",
                val: id_str,
            });
        } else if let Some(_) = id {
            return Err(HtomlError::NonStringID);
        }

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
            let mut attrs = Vec::new();
            if elem_type == "a" {
                Self::parse_a_element(&mut attrs, table)?;
            }
            let mut class_str = String::new();
            Self::parse_class_and_id(&mut attrs, table, &mut class_str)?;
            match elem_type {
                void if VOID_ELEMENT.contains(&void) => html.insert_void_elem(void),
                real => {
                    html.open_elem_with_args(real, &attrs);
                    Self::parse_element(html, elem_cont?)?;
                    html.close_elem(real);
                }
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
