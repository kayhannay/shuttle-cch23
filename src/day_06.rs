use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use tracing::info;

pub async fn day06_post(text: String) -> Result<Json<Answer>, StatusCode> {
    info!("Got text: {}", text);
    let elfs: i32 = text.split(" ").filter(|word| word.contains(&"elf")).count() as i32;
    let mut filter_text = text.clone();
    let mut elf_shelf = 0;
    while filter_text.find("elf on a shelf").is_some() {
        filter_text = filter_text.replacen("elf on a shelf", "", 1);
        elf_shelf += 1;
    }
    let shelf_no_elf: i32 = filter_text.split(" ").filter(|word| word.contains(&"shelf")).count() as i32;
    let answer = Answer {
        elf: elfs,
        elf_shelf,
        shelf_no_elf,
    };
    Ok(Json(answer))
}

#[derive(Serialize, Debug)]
pub struct Answer {
    pub elf: i32,
    #[serde(rename = "elf on a shelf")]
    pub elf_shelf: i32,
    #[serde(rename = "shelf with no elf on it")]
    pub shelf_no_elf: i32,
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::day_06::Answer;

    #[tokio::test]
    pub async fn test_day06_post() {
        let answer = super::day06_post("The mischievous elf peeked out from behind the toy workshop,
                             and another elf joined in the festive dance.
                             Look, there is also an elf on that shelf!".to_string()).await.expect("Should be ok");
        let a: &Answer = answer.deref();

        assert_eq!(a.elf, 4);
    }

    #[tokio::test]
    pub async fn test_day06_contest_post() {
        let answer = super::day06_post("there is an elf on a shelf on an elf.
                                                    there is also another shelf in Belfast.".to_string()
                                                    ).await.expect("Should be ok");
        let a: &Answer = answer.deref();

        assert_eq!(a.elf, 5);
        assert_eq!(a.elf_shelf, 1);
        assert_eq!(a.shelf_no_elf, 1);
    }
}
