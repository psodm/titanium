use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Resource {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: String,
    pub emp_type: String,
    pub manager: String,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [{}], {}, Type: {}, Manager: {}", self.name, self.email, self.role, self.emp_type, self.manager)
    }
}
