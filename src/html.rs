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

    pub fn open_elem(&mut self, elem: &str) {
        self.0.push('<');
        self.0.push_str(elem);
        self.0.push('>');
    }

    pub fn close_elem(&mut self, elem: &str) {
        self.0.push_str("</");
        self.0.push_str(elem);
        self.0.push('>');
    }

    pub fn insert_elem(&mut self, elem: &str, cont: &str) {
        self.open_elem(elem);
        self.0.push_str(cont);
        self.close_elem(elem);
    }

    pub fn insert_void_elem(&mut self, elem: &str) {
        self.0.push('<');
        self.0.push_str(elem);
        self.0.push_str(" />");
    }
}
