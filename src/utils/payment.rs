use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref AMEX: Regex = Regex::new("^3[47][0-9]{13}$").unwrap();
    static ref DISCOVER: Regex = Regex::new("^6(?:011|[45][0-9]{2})[0-9]{12}$").unwrap();
    static ref JCB: Regex = Regex::new("^35(?:[2-8][0-9]|9[0-1])[0-9]{11}$").unwrap();
    static ref DINERS: Regex = Regex::new("^3(?:0[0-5]|[68][0-9])[0-9]{11}$").unwrap();
    static ref UNIONPAY: Regex = Regex::new("^62[0-9]{14,17}$").unwrap();
    static ref VISA: Regex = Regex::new("^4[0-9]{12}(?:[0-9]{3})?$").unwrap();
    static ref MASTERCARD: Regex = Regex::new("^5[1-5][0-9]{14}$").unwrap();
}

pub fn categorize_card_number(card_number: &str) -> String {
    if VISA.is_match(card_number) {
        return String::from("Visa");
    }

    if MASTERCARD.is_match(card_number) {
        return String::from("Mastercard");
    }

    if AMEX.is_match(card_number) {
        return String::from("American Express");
    }

    if DISCOVER.is_match(card_number) {
        return String::from("Discover");
    }

    if JCB.is_match(card_number) {
        return String::from("JCB");
    }

    if DINERS.is_match(card_number) {
        return String::from("Diners Club/Carte Blanche");
    }

    if UNIONPAY.is_match(card_number) {
        return String::from("UnionPay");
    }

    return String::from("unknown");
}

pub fn get_last_4_digits(number: &str) -> String {
    return number.get(number.len() - 4..).unwrap().to_owned();
}
