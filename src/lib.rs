use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::TelegramApi;
use frankenstein::Api;
use frankenstein::KeyboardButton;
use frankenstein::ReplyKeyboardMarkup;
use frankenstein::ReplyMarkup;

pub mod database;

pub use crate::database::*;

pub static TOKEN: &str = "TOKEN";

pub static COMMANDS: [&str; 3] = ["/check", "/start", "/commands"];
pub static mut NEED_CHECK: bool = false;

pub fn set_keyboard_markup() -> ReplyKeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = Vec::new();
    let mut row_check: Vec<KeyboardButton> = Vec::new();
    let mut row_commands: Vec<KeyboardButton> = Vec::new();

    let check_button = KeyboardButton::builder().text("/check").build();
    row_check.push(check_button);

    let commands_button = KeyboardButton::builder().text("/commands").build();
    row_commands.push(commands_button);

    keyboard.push(row_commands);
    keyboard.push(row_check);
    let keyboard_markup = ReplyKeyboardMarkup::builder().keyboard(keyboard).build();

    keyboard_markup
}

pub fn send_message(message: Message, api: &Api, keyboard_markup: &ReplyKeyboardMarkup) -> SendMessageParams {
    let mut username = String::new();
    let send_message_params;
    let message_text;
                        
    match api.get_me() {
        Ok(response) => username = response.result.username.expect("Got no username"),
        Err(error) => eprintln!("Failed to get me: {error:?}"),
    }

    if message.clone().text == None {
        message_text = format!("Не понимаю тебя");
    }

    else if message.clone().text.unwrap() == "/start" {
        unsafe {
            NEED_CHECK = false;
        }
        message_text = format!("Привет, я @{}", username);
    }

    else if message.clone().text.unwrap() == "/check" {
        unsafe {
            NEED_CHECK = true;
        }
        message_text = "Что ты хочешь проверить?".to_string();
    }

    else if message.clone().text.unwrap() == "/commands" {
        unsafe {
            NEED_CHECK = false;
        }
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

        let request = message.clone().text.unwrap().to_lowercase();

        unsafe {
            if NEED_CHECK {
                let response = Database::get_record(request);

                if !response.is_empty() {
                    message_text = response;
                }
                else {
                    message_text = format!("Не могу найти предмет '{}' в моей базе данных", message.clone().text.unwrap());
                }
            }
            else {
                message_text = "Не понимаю тебя".to_string();
            }
        }
    }

    else {
        unsafe {
            NEED_CHECK = false;
        }
        message_text = "Не понимаю тебя".to_string();
    }

    send_message_params = SendMessageParams::builder()
    .chat_id(message.chat.id)
    .text(message_text)
    .reply_markup(ReplyMarkup::ReplyKeyboardMarkup(keyboard_markup.clone()))
    .build();

    send_message_params
}

