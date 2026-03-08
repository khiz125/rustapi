use crate::domain::user::vo::{Email, UserId, UserName};

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: Email,
}

impl User {
    pub fn new(name: UserName, email: Email) -> Self {
        Self {
            id: UserId::new(0),
            name,
            email,
        }
    }

    pub fn with_id(id: UserId, name: UserName, email: Email) -> Self {
        Self { id, name, email }
    }
}
