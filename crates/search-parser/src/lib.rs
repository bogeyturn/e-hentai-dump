mod prefixes;
mod tokens;

#[derive(Debug)]
pub enum Operator {
    Exclude,
    Or,
}

#[derive(Debug)]
pub enum Namespace {
    Artist,
    Character,
    Cosplayer,
    Female,
    Male,
    Group,
    Mixed,
    Other,
    Parody,
    Reclass,
    Language,
    Location,
    Tag,
    Title,
    Uploader,
    UploadUID,
    GID,
    Comment,
    Favnote,
    Unknown(String),
}

#[derive(Debug)]
pub struct SearchTerm {
    pub operator: Option<Operator>,
    pub namespace: Option<Namespace>,
    pub value: Value,
    pub exact: bool,
}

fn connect(items: Vec<SearchTerm>) -> Vec<SearchTerm> {
    let mut builder: Vec<SearchTerm> = vec![];
    let mut merge = false;
    for item in items {
        if !merge && item.namespace.is_none() && item.operator.is_none() && item.value.len() < 3 {
            if item.value.len() != 0 {
                builder.push(item);
            }
            merge = true;
        } else {
            if merge {
                merge = false;
                if item.namespace.is_none() && item.operator.is_none() {
                    let l = builder.last_mut().unwrap();
                    l.value = l.value.join(&item.value);
                } else {
                    if item.value.len() != 0 {
                        builder.push(item);
                    }
                }
            } else {
                if item.value.len() != 0 {
                    builder.push(item);
                }
            }
        }
    }
    builder
}

pub fn parse_search(input: &str) -> Vec<SearchTerm> {
    let tokens = tokens::parse_tokens(input);
    let tokens = tokens
        .iter()
        .map(|v| (prefixes::parse_op(&v.0), v.1))
        .map(|((op, input), exact)| {
            let v = prefixes::parse_prefix(input);
            match v {
                Some((ns, value)) => SearchTerm {
                    operator: op,
                    namespace: Some(ns),
                    value: Value::from(value),
                    exact,
                },
                None => SearchTerm {
                    operator: op,
                    namespace: None,
                    value: Value::from(input),
                    exact,
                },
            }
        })
        .collect::<Vec<_>>();
    connect(tokens)
}

#[derive(Debug)]
pub enum Value {
    Exact(String),
    Wildcard(String),
    None(String),
}

impl Value {
    pub fn len(&self) -> usize {
        match self {
            Value::Exact(s) => s.len(),
            Value::Wildcard(s) => s.len(),
            Value::None(s) => s.len(),
        }
    }

    pub fn str(&self) -> String {
        match self {
            Value::Exact(s) => s.clone(),
            Value::Wildcard(s) => s.clone(),
            Value::None(s) => s.clone(),
        }
    }

    pub fn join(&self, other: &Self) -> Self {
        let s = self.str();
        match other {
            Value::Exact(o) => Value::Exact(format!("{} {}", s, o)),
            Value::Wildcard(o) => Value::Wildcard(format!("{} {}", s, o)),
            Value::None(o) => Value::None(format!("{} {}", s, o)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        let s = s.replace("_", " ");
        if s.ends_with('*') || s.ends_with('%') {
            Value::Wildcard(s[..s.len() - 1].to_string())
        } else if s.ends_with('$') {
            Value::Exact(s[..s.len() - 1].to_string())
        } else {
            Value::None(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_search;

    #[test]
    fn aaa() {
        let input = r#"-artist:a a dick weak:ohaaa:aa tag:rimjob% abcd_aaa fdsjijf-aa "exact search" male:m test* aaijd$ ~other:o male:"acdcuhds siuh" "#;
        let terms = parse_search(input);
        for term in terms {
            println!("{:?}", term);
        }
    }
}
