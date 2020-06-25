use crate::schema::*;

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
}
