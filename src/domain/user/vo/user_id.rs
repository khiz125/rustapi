#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(i32);

impl UserId {
    pub fn new(id: i32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
