pub struct Item {
    raw: Vec<u8>,
}

impl From<Item> for Vec<u8> {
    fn from(i: Item) -> Self {
        i.raw
    }
}

impl From<Vec<u8>> for Item {
    fn from(raw: Vec<u8>) -> Self {
        Self { raw }
    }
}

#[derive(Clone)]
pub struct Name {
    name: String,
}
impl From<String> for Name {
    fn from(name: String) -> Self {
        Self { name }
    }
}
impl From<Name> for String {
    fn from(n: Name) -> Self {
        n.name
    }
}

pub struct NamedItem {
    item: Item,
    name: Name,
}

impl NamedItem {
    pub fn new(item: Item, name: Name) -> Self {
        Self { item, name }
    }

    pub fn as_name(&self) -> &Name {
        &self.name
    }
}

impl From<NamedItem> for Item {
    fn from(named: NamedItem) -> Self {
        named.item
    }
}
