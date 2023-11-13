use crate::arg::Argument;

#[derive(Debug, Default)]
pub struct Html(String);

impl From<Html> for String {
    fn from(value: Html) -> Self {
        value.0
    }
}

impl Html {
    pub fn push(&mut self, cont: &str) {
        self.0.push_str(cont);
    }

    pub fn open_elem_with_args(&mut self, elem: &str, args: &[Argument<'_>]) {
        self.0.push('<');
        self.0.push_str(elem);
        for arg in args {
            self.0.push(' ');
            self.0.push_str(arg.name);
            self.0.push('=');
            self.0.push('"');
            self.0.push_str(arg.val);
            self.0.push('"');
        }
        self.0.push('>');
    }

    pub fn open_elem(&mut self, elem: &str) {
        self.open_elem_with_args(elem, &[]);
    }

    pub fn close_elem(&mut self, elem: &str) {
        self.0.push_str("</");
        self.0.push_str(elem);
        self.0.push('>');
    }

    pub fn insert_elem_with_args(&mut self, elem: &str, cont: &str, args: &[Argument<'_>]) {
        self.open_elem_with_args(elem, args);
        self.0.push_str(cont);
        self.close_elem(elem);
    }

    pub fn insert_elem(&mut self, elem: &str, cont: &str) {
        self.insert_elem_with_args(elem, cont, &[]);
    }

    pub fn insert_void_elem(&mut self, elem: &str) {
        self.0.push('<');
        self.0.push_str(elem);
        self.0.push_str(" />");
    }
}
