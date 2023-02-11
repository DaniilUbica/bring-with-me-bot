use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::SendPhotoParams;
use frankenstein::TelegramApi;
use frankenstein::Api;
use frankenstein::KeyboardButton;
use frankenstein::ReplyKeyboardMarkup;
use frankenstein::ReplyMarkup;

use database::Database::get_item_names;
use send_info::send_info::write_item;

pub mod database;
pub mod send_info;

pub use crate::database::*;
pub use crate::send_info::*;

pub static TOKEN: &str = "TOKEN";

pub static COMMANDS: [&str; 2] = ["/start", "/commands"];

pub fn set_keyboard_markup() -> ReplyKeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();
    let mut row_commands: Vec<KeyboardButton> = Vec::new();

    let commands_button = KeyboardButton::builder().text("/commands").build();
    row_commands.push(commands_button);

    keyboard.push(row_commands);
    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();

    keyboard_markup
}

pub fn send_message(message: Message, api: &Api, keyboard_markup: &ReplyKeyboardMarkup) -> (SendMessageParams, SendPhotoParams) {
    let send_message_params;
    let send_photo_params;
    let message_text;
    let photo_path;
    
    let mut username = String::new();
    let mut request = message.clone().text.unwrap().to_lowercase();
    
    match api.get_me() {
        Ok(response) => username = response.result.username.expect("Got no username"),
        Err(error) => eprintln!("Failed to get me: {error:?}"),
    }
    
    if message.clone().text == None {
        message_text = format!("Не понимаю тебя");
        photo_path = std::path::PathBuf::from(format!("./Photos/question.png"));
    }

    else if message.clone().text.unwrap() == "/start" {
        message_text = format!("Привет, я @{}\nПросто напиши мне, что ты хочешь взять с собой\n\nHi, i am @{}\nJust text me what you want to take with you", username, username);
        photo_path = std::path::PathBuf::from(format!("./Photos/logo.png"));
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
        photo_path = std::path::PathBuf::from(format!("./Photos/commands.png"));
    }

    else if !COMMANDS.contains(&&message.clone().text.unwrap()[..]) {

        if message.clone().text.unwrap().chars().nth(0).unwrap() != '/' {

            let response = Database::get_record(request.clone());
            let (rus_request, eng_request) = get_item_names(&request);

            if request == rus_request {
                request = eng_request;
            }

            if !response.is_empty() {
                message_text = response.clone();
                photo_path = std::path::PathBuf::from(format!("./Photos/{}.png", request.clone().trim()));
            }
            else {
                message_text = format!("Не могу найти предмет '{}' в моей базе данных", message.clone().text.unwrap());
                photo_path = std::path::PathBuf::from(format!("./Photos/dont_know.png"));
                write_item(request);
            }
        }
        else {
            message_text = "Не понимаю тебя".to_string();
            photo_path = std::path::PathBuf::from(format!("./Photos/question.png"));
        }
    }

    else {
        message_text = "Не понимаю тебя".to_string();
        photo_path = std::path::PathBuf::from(format!("./Photos/question.png"));
    }

    send_message_params = SendMessageParams::builder()
    .chat_id(message.chat.id)
    .text(message_text)
    .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup.clone()))
    .build();

    send_photo_params = SendPhotoParams::builder()
    .chat_id(message.clone().chat.id)
    .photo(photo_path)
    .build();

    (send_message_params, send_photo_params)
}

