struct PropertyInstance {
   pub name: String,
   pub entries: Vec<PropertyEntry>
}

struct PropertyEntry {
    pub permissions: String,
    pub encoding: String,
    pub comment: String,
    pub content: String,
}