use axum::http::StatusCode;
use axum::Json;
use axum::routing::post;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha2::Digest;
use tracing::info;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/nice", post(day15_password))
        .route("/game", post(day15_game))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Data {
    pub input: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Result {
    Nice,
    Naughty,
}

impl Result {
    fn as_str(&self) -> String {
        match self {
            Result::Nice => "nice".to_string(),
            Result::Naughty => "naughty".to_string()
        }
    }

    fn from_str(s: &str) -> Result {
        match s {
            "nice" => Result::Nice,
            _ => Result::Naughty
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Response {
    pub result: String,
}

async fn day15_password(Json(data): Json<Data>) -> (StatusCode, Json<Response>) {
    let password = data.input;
    info!("Password nice: {}", password);
    let result = match password.as_str() {
        password if password.contains("ab") => Result::Naughty.as_str(),
        password if password.contains("cd") => Result::Naughty.as_str(),
        password if password.contains("pq") => Result::Naughty.as_str(),
        password if password.contains("xy") => Result::Naughty.as_str(),
        password if password.chars().filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')).count() < 3 => Result::Naughty.as_str(),
        password if password.chars().zip(password.chars().skip(1)).any(|(a, b)| a.is_alphabetic() && a == b) => Result::Nice.as_str(),
        _ => Result::Naughty.as_str(),
    };
    match Result::from_str(result.as_str()) {
        Result::Nice => (StatusCode::OK, Json(Response { result })),
        Result::Naughty => (StatusCode::BAD_REQUEST, Json(Response { result })),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GameResponse {
    pub result: String,
    pub reason: String,
}

async fn day15_game(Json(data): Json<Data>) -> (StatusCode, Json<GameResponse>) {
    let password = data.input;
    println!("Password game: {}", password);
    let uppercase = Regex::new(r"[A-Z]+").unwrap();
    let lowercase = Regex::new(r"[a-z]+").unwrap();
    let digit = Regex::new(r"[0-9]").unwrap();
    let number = Regex::new(r"[0-9]+").unwrap();
    let unicode = Regex::new(r"[\u{2980}-\u{2BFF}]").unwrap();
    let emoji = Regex::new(concat!(
        "[",
        "\u{01F600}-\u{01F64F}", // emoticons
        "\u{01F300}-\u{01F5FF}", // symbols & pictographs
        "\u{01F680}-\u{01F6FF}", // transport & map symbols
        "\u{01F1E0}-\u{01F1FF}", // flags (iOS)
        "\u{002702}-\u{0027B0}",
        //"\u{0024C2}-\u{01F251}",
        "]+",
    )).unwrap();
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, &password);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);
    info!("Hash: {}", hash_hex);
    info!("Math: {}", number.find_iter(password.as_str()).map(|m| m.as_str().parse::<i32>().unwrap()).sum::<i32>());
    info!("Emojis: {:?}", emoji.find_iter(&password).collect::<Vec<_>>());
    match password.as_str() {
        password if password.len() < 8 => (StatusCode::BAD_REQUEST, Json(GameResponse { result: Result::Naughty.as_str(), reason: "8 chars".to_string() })),
        password if !uppercase.is_match(password) || !lowercase.is_match(password) || !digit.is_match(password) => (StatusCode::BAD_REQUEST, Json(GameResponse { result: Result::Naughty.as_str(), reason: "more types of chars".to_string() })),
        password if digit.find_iter(password).count() < 5 => (StatusCode::BAD_REQUEST, Json(GameResponse { result: Result::Naughty.as_str(), reason: "55555".to_string() })),
        password if number.find_iter(password).map(|m| m.as_str().parse::<i32>().unwrap()).sum::<i32>() != 2023 => (StatusCode::BAD_REQUEST, Json(GameResponse { result: Result::Naughty.as_str(), reason: "math is hard".to_string() })),
        password if !contains_joy(password) => (StatusCode::NOT_ACCEPTABLE, Json(GameResponse { result: Result::Naughty.as_str(), reason: "not joyful enough".to_string() })),
        password if !password.chars().zip(password.chars().skip(1).zip(password.chars().skip(2))).any(|(a, (b, c))| a.is_alphabetic() && a == c && b != a) => (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, Json(GameResponse { result: Result::Naughty.as_str(), reason: "illegal: no sandwich".to_string() })),
        password if !unicode.is_match(password) => (StatusCode::RANGE_NOT_SATISFIABLE, Json(GameResponse { result: Result::Naughty.as_str(), reason: "outranged".to_string() })),
        password if !emoji.is_match(password) => (StatusCode::UPGRADE_REQUIRED, Json(GameResponse { result: Result::Naughty.as_str(), reason: "😳".to_string() })),
        _password if !hash_hex.ends_with("a") => (StatusCode::IM_A_TEAPOT, Json(GameResponse { result: Result::Naughty.as_str(), reason: "not a coffee brewer".to_string() })),
        _ => (StatusCode::OK, Json(GameResponse { result: Result::Nice.as_str(), reason: "that's a nice password".to_string() })),
    }
}

fn contains_joy(password: &str) -> bool {
    let j = password.match_indices("j");
    let o = password.match_indices("o");
    let y = password.match_indices("y");
    if j.clone().count() != 1 || o.clone().count() != 1 || y.clone().count() != 1 {
        println!("Does not contain joy, too many characters.");
        return false;
    }
    if j.min() < o.clone().min() && o.min() < y.min() {
        println!("Contains joy.");
        return true;
    }
    println!("Does not contain joy, wrong order.");
    false
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use rstest::*;

    //#[tokio::test]
    #[rstest]
    #[case::ok("vattyru", StatusCode::OK, "nice")]
    #[case::three_vowels("aaa", StatusCode::OK, "nice")]
    #[case::two_vowels("utatb", StatusCode::BAD_REQUEST, "naughty")]
    #[case::ab("abaa", StatusCode::BAD_REQUEST, "naughty")]
    #[case::ab("cdaa", StatusCode::BAD_REQUEST, "naughty")]
    #[case::ab("pqaa", StatusCode::BAD_REQUEST, "naughty")]
    #[case::ab("xyaa", StatusCode::BAD_REQUEST, "naughty")]
    async fn test_day15_password2(#[case] password: String, #[case] expected_status: StatusCode, #[case] expected_result: String) {
        use super::*;
        let data = Data { input: password };
        let (status, response) = day15_password(Json(data)).await;
        assert_eq!(status, expected_status);
        assert_eq!(response.result, expected_result);
    }

    #[rstest]
    #[case::not_8_characters("mario", StatusCode::BAD_REQUEST, "naughty", "8 chars")]
    #[case::not_enough_types_1("mariobro", StatusCode::BAD_REQUEST, "naughty", "more types of chars")]
    #[case::not_enough_types_2("mariobro", StatusCode::BAD_REQUEST, "naughty", "more types of chars")]
    #[case::not_enough_types_3("EEEEEEEEEEE", StatusCode::BAD_REQUEST, "naughty", "more types of chars")]
    #[case::not_enough_types_4("E3E3E3E3E3E", StatusCode::BAD_REQUEST, "naughty", "more types of chars")]
    #[case::five_digits("e3E3e#eE#ee3#EeE3", StatusCode::BAD_REQUEST, "naughty", "55555")]
    #[case::sum_2023_1("Password12345", StatusCode::BAD_REQUEST, "naughty", "math is hard")]
    #[case::sum_2023_2("2 00 2 3 OOgaBooga", StatusCode::BAD_REQUEST, "naughty", "math is hard")]
    #[case::joy_1("2+2/2-8*8 = 1-2000 OOgaBooga", StatusCode::NOT_ACCEPTABLE, "naughty", "not joyful enough")]
    #[case::joy_2("2000.23.A yoyoj", StatusCode::NOT_ACCEPTABLE, "naughty", "not joyful enough")]
    #[case::joy_3("2000.23.A joy joy", StatusCode::NOT_ACCEPTABLE, "naughty", "not joyful enough")]
    #[case::joy_4("2000.23.A joyo", StatusCode::NOT_ACCEPTABLE, "naughty", "not joyful enough")]
    #[case::sandwich_1("2000.23.A j  ;)  o  ;)  y", StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, "naughty", "illegal: no sandwich")]
    #[case::sandwich_2("2020.3.A j  ;)  o  ;)  y", StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, "naughty", "illegal: no sandwich")]
    #[case::unicode_range_1("2000.23.A j  ;)  o  ;)  y AzA", StatusCode::RANGE_NOT_SATISFIABLE, "naughty", "outranged")]
    #[case::unicode_range_2("2000.23.A j  ;)  o  ;)  y⥿ AzA", StatusCode::RANGE_NOT_SATISFIABLE, "naughty", "outranged")]
    #[case::emoji("2000.23.A j  ;)  o  ;)  y ⦄AzA", StatusCode::UPGRADE_REQUIRED, "naughty", "😳")]
    #[case::hash("2000.23.A j  🥶  o  🍦  y ⦄AzA", StatusCode::IM_A_TEAPOT, "naughty", "not a coffee brewer")]
    #[case::ok("2000.23.A j ⦖⦖⦖⦖⦖⦖⦖⦖ 🥶  o  🍦  y ⦄AzA", StatusCode::OK, "nice", "that's a nice password")]
    async fn test_day15_game(#[case] password: String, #[case] expected_status: StatusCode, #[case] expected_result: String, #[case] expected_reason: String) {
        use super::*;
        let data = Data { input: password };
        let (status, response) = day15_game(Json(data)).await;
        assert_eq!(status, expected_status);
        assert_eq!(response.result, expected_result);
        assert_eq!(response.reason, expected_reason);
    }
}