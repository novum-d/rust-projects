#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

#[allow(dead_code)]
impl FullName {
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        FullName {
            first_name: first_name.into(),
            last_name: last_name.into(),
        }
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }
}
