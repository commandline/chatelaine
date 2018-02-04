use argon2rs;
use base64;
use rand::{self, Rng};
use super::schema::credentials;

#[derive(Debug, Queryable)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name = "credentials"]
pub struct NewCredentials {
    pub username: String,
    pub password: String,
    pub salt: String,
    pub admin: bool,
}

impl NewCredentials {
    pub fn new(username: &str, password: &str) -> Self {
        let mut salt = vec![0; 32];
        let mut rng = rand::thread_rng();

        for c in salt.iter_mut() {
            *c = rng.gen();
        }
        let salt = base64::encode(&salt);
        let password = argon2rs::argon2i_simple(password, &salt);
        let password = base64::encode(&password);
        NewCredentials {
            username: username.to_owned(),
            password,
            salt,
            admin: false,
        }
    }
}
