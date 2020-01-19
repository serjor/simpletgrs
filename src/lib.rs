use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

pub fn send_photo(
    photo: &str,
    caption: &str,
    token: &str,
    group: &str,
) -> Result<TelegramResponseMessage, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut map = HashMap::new();
    map.insert("chat_id", group);
    map.insert("photo", photo);
    map.insert("caption", caption);
    map.insert("parse_mode", "HTML");
    let url =
        reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/sendPhoto", token)).unwrap();
    let response = client.post(url).json(&map).send()?;
    let message: TelegramResponseMessage = response.json()?;

    Ok(message)
}

#[derive(Deserialize)]
struct TelegramBot {
    ok: bool,
    result: TelegramMe,
}

#[derive(Deserialize)]
struct TelegramMe {
    id: i64,
}

pub fn get_me(token: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let url = reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/getMe", token)).unwrap();

    let me: TelegramBot = reqwest::blocking::get(url)?.json()?;

    if me.ok == false {
        return Ok(0);
    }

    Ok(me.result.id)
}

#[derive(Deserialize, Debug)]
pub struct TelegramUser {
    id: i64,
}

#[derive(Deserialize, Debug)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub user: Option<TelegramUser>,
    pub date: i64,
    pub chat: TelegramChat,
}

#[derive(Deserialize, Debug)]
pub struct TelegramResponseMessage {
    pub ok: bool,
    pub result: TelegramMessage,
}

#[derive(Deserialize, Debug)]
pub struct TelegramUpdate {
    update_id: i64,
    message: Option<TelegramMessage>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramUpdates {
    ok: bool,
    result: Option<Vec<TelegramUpdate>>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramChat {
    pub id: i64,
}

pub fn get_updates(token: &str) -> Result<Vec<TelegramUpdate>, Box<dyn std::error::Error>> {
    let url =
        reqwest::Url::parse(&format!("https://api.telegram.org/bot{}/getUpdates", token)).unwrap();

    let updates: TelegramUpdates = reqwest::blocking::get(url)?.json()?;

    Ok(updates.result.unwrap())
}

pub fn pin_chat_message(
    message: TelegramMessage,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = reqwest::Url::parse(&format!(
        "https://api.telegram.org/bot{}/pinChatMessage",
        token
    ))
    .unwrap();

    let mut map = HashMap::new();
    map.insert("chat_id", message.chat.id);
    map.insert("message_id", message.message_id);

    client.post(url).json(&map).send()?;

    Ok(())
}
