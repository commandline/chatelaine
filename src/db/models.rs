use super::schema::credentials;

#[derive(Queryable)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "credentials"]
pub struct NewCredentials {
    pub username: String,
    pub password: String,
}
