use frankenstein::GetUpdatesParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use std::thread;

use tg_bot::Database;
use tg_bot::send_info::send_info;

fn main() {
    let api = Api::new(tg_bot::TOKEN);

    Database::create_table();

    Database::add_record(["нож", "knife", "rus_info", "eng_info", "rus_allowed", "eng_allowed"].to_vec());

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let keyboard_markup = tg_bot::set_keyboard_markup();

    loop {
        let result = api.get_updates(&update_params);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {

                        let api_clone = api.clone();

                        let keyboard_clone = keyboard_markup.clone();

                        if message.clone().text.unwrap() == "send" {
                            send_info::send_items_info();
                        }

                        thread::spawn(move || {
                            let (send_message_params, send_photo_params) = tg_bot::send_message(message, &api_clone, &keyboard_clone);

                            if let Err(err) = api_clone.send_message(&send_message_params) {
                                eprintln!("Failed to send message: {err:?}");
                            }

                            if let Err(err) = api_clone.send_photo(&send_photo_params) {
                                eprintln!("Failed to upload photo: {err:?}");
                            } 
                            
                        });
                    }
                    update_params = update_params_builder
                        .clone()
                        .offset(update.update_id + 1)
                        .build();
                }
            }
            Err(error) => {
                println!("Failed to get updates: {error:?}");
            }
        }
    }

}
