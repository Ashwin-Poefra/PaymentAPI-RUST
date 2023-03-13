use lazy_static::lazy_static;

lazy_static! {
  pub static ref RSA_KEY_ID: String = std::env::var("KMS_RSA_KEY_ID").unwrap_or("".to_string());
}