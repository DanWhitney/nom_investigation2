
// <name> something </name>

use std::collections::{HashMap, HashSet};

enum XmlParts {
  NameStart{name: String, depth: usize},
  NameEnd{name: String, depth: usize},
  Attribute{key: String, value: String},
}

struct XmlElement {
  name: String,
  depth: usize,
  attributes: HashMap<String,String>,
  children: Vec<XmlElement>
}