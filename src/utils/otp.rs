use crate::error::{Error, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use rand::prelude::*;
use rand::rng;

pub trait CheckOTP {
    fn get_hashed_otp(&self) -> Option<&str>;

    fn check_otp(&self, otp: String) -> Result<bool> {
        if let Some(hash) = self.get_hashed_otp() {
            match verify(otp, hash) {
                Ok(ok) => {
                    if ok {
                        return Ok(true);
                    } else {
                        return Err(Error::new("otp not match"));
                    }
                }
                Err(_) => return Err(Error::new("failed to verify the otp")),
            }
        }

        Err(Error::new("otp not available"))
    }
}

pub fn hash_otp(otp: String) -> Result<String> {
    hash(otp, DEFAULT_COST).map_err(|_| Error::new("failed to hash otp"))
}

pub fn generate_otp() -> String {
    let mut rng = rng();
    let x = rng.random_range(1000..9999);
    format!("{}", x)
}
