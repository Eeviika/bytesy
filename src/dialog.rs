#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogElement {
    Dialog { text: String },
    PageBreak,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dialog {
    name: String,
    dialogue: Vec<DialogElement>,
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog { name: "dialogue".to_string(), dialogue: vec![DialogElement::Dialog { text: "Hello, World!".to_string() }] }
    }
}
