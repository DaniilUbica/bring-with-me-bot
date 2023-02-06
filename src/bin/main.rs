use frankenstein::GetUpdatesParams;
use frankenstein::TelegramApi;
use frankenstein::{Api, UpdateContent};
use tg_bot::Database;

fn main() {
    let api = Api::new(tg_bot::TOKEN);

    Database::create_table();

    Database::add_record(["нож", "knife", "Бытовые ножи и ножницы можно провозить в багаже. Правда, если вам захочется провезти тесак, возможно, его отправят на экспертизу, не холодное ли оно оружие. А вот в ручной клади нельзя провозить даже складные (без фиксатора) дорожные, перочинные ножи с длиной лезвия менее 60 мм.", "Household knives and scissors can be carried in luggage. However, if you want to bring a cleaver, perhaps it will be sent for examination, whether it is a cold weapon. But in hand luggage it is impossible to carry even folding (without a lock) travel, penknives with a blade length of less than 60 mm."].to_vec());

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let keyboard_markup = tg_bot::set_keyboard_markup();

    loop {
        let result = api.get_updates(&update_params);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {

                        let send_message_params = tg_bot::send_message(message, &api, &keyboard_markup);
                        
                        if let Err(err) = api.send_message(&send_message_params) {
                            println!("Failed to send message: {err:?}");
                        }
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
