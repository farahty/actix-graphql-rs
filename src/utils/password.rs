use crate::error::{Error, Result};
use bcrypt::{DEFAULT_COST, hash, verify};

pub trait CheckPassword {
    fn get_hashed_password(&self) -> Option<String>;

    fn check_password(&self, password: String) -> Result<bool> {
        if let Some(hash) = self.get_hashed_password() {
            match verify(password, &hash) {
                Ok(ok) => {
                    if ok {
                        return Ok(true);
                    } else {
                        return Err(Error::new("password not match"));
                    }
                }
                Err(_) => return Err(Error::new("failed to verify the password")),
            }
        }

        Err(Error::new("password not available"))
    }
}

pub trait HashPassword {
    fn get_password(&self) -> Option<String>;
    fn set_password(&mut self, password: String);

    fn hash_password(&mut self) -> Result<bool> {
        if let Some(password) = self.get_password() {
            let hashed = match hash(password, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(_) => return Err(Error::new("failed to hash password")),
            };

            self.set_password(hashed)
        }

        Ok(true)
    }
}
