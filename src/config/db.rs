use lazy_static::lazy_static;

lazy_static! {
  pub static ref PAYMENTS_TABLE_NAME: String = std::env::var("PAYMENTS_TABLE_NAME").unwrap_or("".to_string());
}