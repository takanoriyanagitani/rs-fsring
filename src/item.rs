use crate::evt::Event;

/// Contains raw bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl From<&[u8]> for Item {
    fn from(bytes: &[u8]) -> Self {
        Self::from(bytes.to_vec())
    }
}

/// Contains name string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    name: String,
}
impl Name {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
}
impl From<String> for Name {
    fn from(name: String) -> Self {
        Self { name }
    }
}
impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self::from(String::from(s))
    }
}
impl From<Name> for String {
    fn from(n: Name) -> Self {
        n.name
    }
}
impl From<u8> for Name {
    fn from(raw: u8) -> Self {
        let name: String = format!("{:02x}", raw);
        Self { name }
    }
}
impl TryFrom<&Name> for u8 {
    type Error = Event;
    fn try_from(n: &Name) -> Result<Self, Self::Error> {
        let s: &str = n.name.as_str();
        u8::from_str_radix(s, 16)
            .map_err(|e| Event::UnexpectedError(format!("Invalid name: {}", e)))
    }
}

/// A named `Item` with `Name`.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn into_pair(self) -> (Name, Item) {
        (self.name, self.item)
    }

    pub fn into_item(self) -> Item {
        self.item
    }
}

impl From<NamedItem> for Item {
    fn from(named: NamedItem) -> Self {
        named.item
    }
}

impl From<NamedItem> for Name {
    fn from(named: NamedItem) -> Self {
        named.name
    }
}
