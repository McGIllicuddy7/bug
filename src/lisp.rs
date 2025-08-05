#[derive(Debug)]
pub enum NodeError {
    None,
}
#[derive(Clone, Debug)]
pub enum Node {
    Value { s: String },
    List { s: Vec<Node> },
}
impl Node {
    pub fn assume_value(&self) -> Option<String> {
        match self {
            Self::Value { s } => Some(s.clone()),
            Self::List { s: _ } => None,
        }
    }
    pub fn assume_list(&self) -> Option<Vec<Node>> {
        match self {
            Self::Value { s: _ } => None,
            Self::List { s } => Some(s.clone()),
        }
    }
}
pub struct TokenList {
    list: Vec<String>,
    idx: usize,
}
impl TokenList {
    pub fn new(v: &str) -> Self {
        let mut list = Vec::new();
        let mut base = String::new();
        let mut in_string = false;
        let mut was_slash = false;
        for i in v.chars() {
            if in_string {
                if i == '"' && !was_slash {
                    base.push(i);
                    if !base.is_empty() {
                        list.push(base);
                    }
                    base = String::new();
                    in_string = false;
                } else if i == '\\' && !was_slash {
                    was_slash = true;
                    continue;
                } else {
                    base.push(i);
                }
            } else if i.is_whitespace() {
                if !base.is_empty() {
                    list.push(base);
                }
                base = String::new();
            } else if i == '(' || i == ')' {
                if !base.is_empty() {
                    list.push(base);
                }
                list.push(i.to_string());
                base = String::new();
            } else if i == '"' {
                if !base.is_empty() {
                    list.push(base);
                }
                base = i.to_string();
                in_string = true;
            } else {
                base.push(i);
            }
            was_slash = false;
        }
        Self { list, idx: 0 }
    }
    pub fn peek(&self) -> Result<String, NodeError> {
        if self.idx >= self.list.len() {
            Err(NodeError::None)
        } else {
            Ok(self.list[self.idx].clone())
        }
    }
    pub fn next_item(&mut self) -> Result<String, NodeError> {
        if self.idx >= self.list.len() {
            Err(NodeError::None)
        } else {
            let out = self.idx;
            self.idx += 1;
            Ok(self.list[out].clone())
        }
    }
    pub fn is_empty(&self) -> bool {
        self.idx >= self.list.len()
    }
}
impl std::iter::Iterator for TokenList {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.next_item().ok()
    }
}
pub fn parse_node(list: &mut TokenList) -> Result<Node, NodeError> {
    let s = list.next_item()?;
    if s == "(" {
        let mut l = Vec::new();
        loop {
            let n = list.peek()?;
            if n == ")" {
                let _ = list.next_item();
                break;
            }
            let node = parse_node(list)?;
            l.push(node);
        }
        Ok(Node::List { s: l })
    } else {
        Ok(Node::Value { s })
    }
}
pub fn parse_string(s: &str) -> Result<Vec<Node>, NodeError> {
    let mut tt = TokenList::new(s);
    let mut out = Vec::new();
    while !tt.is_empty() {
        let p = parse_node(&mut tt)?;
        out.push(p);
    }
    Ok(out)
}
