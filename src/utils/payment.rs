pub fn mask_payment_card(number: String) -> String {
  let suffix = number.get(number.len() - 4..).unwrap().to_owned();

  return format!("****{}", suffix);
}