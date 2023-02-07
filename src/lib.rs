use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::AsyncTelegramApi;
use frankenstein::AsyncApi;
use frankenstein::KeyboardButton;
use frankenstein::ReplyKeyboardMarkup;
use frankenstein::ReplyMarkup;
use send_info::SendInfo::write_item;

pub mod database;
pub mod send_info;

pub use crate::database::*;
pub use crate::send_info::*;

pub static TOKEN: &str = "TOKEN";

pub static COMMANDS: [&str; 2] = ["/start", "/commands"];

pub async fn set_keyboard_markup() -> ReplyKeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();
    let mut row_commands: Vec<KeyboardButton> = Vec::new();

    let commands_button = KeyboardButton::builder().text("/commands").build();
    row_commands.push(commands_button);

    keyboard.push(row_commands);
    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();

    keyboard_markup
}

pub async fn send_message(message: Message, api: &AsyncApi, keyboard_markup: &ReplyKeyboardMarkup) -> SendMessageParams {
    let mut username = String::new();
    let send_message_params;
    let message_text;
                        
    match api.get_me().await {
        Ok(response) => username = response.result.username.expect("Got no username"),
        Err(error) => eprintln!("Failed to get me: {error:?}"),
    }

    if message.clone().text == None {
        message_text = format!("Не понимаю тебя");
    }

    else if message.clone().text.unwrap() == "/start" {
        message_text = format!("Привет, я @{}", username);
    }

    else if message.clone().text.unwrap() == "/commands" {
        let mut commands = String::from("Команды, которые я знаю: \n");

        for command in COMMANDS {
            if command == "/start" {
                continue;
            }
            commands += command;
            commands += "\n";
        }

        message_text = commands;
    }

    else if !COMMANDS.contains(&&message.clone().text.unwrap()[..]) {

        if message.clone().text.unwrap().chars().nth(0).unwrap() != '/' {

        let request = message.clone().text.unwrap().to_lowercase();

            let response = Database::get_record(request.clone());

            if !response.is_empty() {
                message_text = response;
            }
            else {
                message_text = format!("Не могу найти предмет '{}' в моей базе данных", message.clone().text.unwrap());
                write_item(request);
            }
        }
        else {
            message_text = "Не понимаю тебя".to_string();
        }
    }

    else {
        message_text = "Не понимаю тебя".to_string();
    }

    send_message_params = SendMessageParams::builder()
    .chat_id(message.chat.id)
    .text(message_text)
    .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup.clone()))
    .build();

    send_message_params
}

