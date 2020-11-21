
pub struct Record<'a> {
    name: String,
    items: Vec<&'a Item<'a>>,
}
pub struct Item<'b> {
    name: String,
    fields: Vec<&'b Field>,
}
pub struct Field {
    name: String,
    kind: FieldKind,
}

pub enum FieldKind {
    Int, String, Age
}

pub struct Data<'a, 'b> {
    input: String,
    records: Vec<&'a Record<'a>>,
    items: Vec<&'b Item<'b>>,
}

