use frankenstein::GetUpdatesParams;
use frankenstein::AsyncTelegramApi;
use frankenstein::{AsyncApi, UpdateContent};
use tg_bot::Database;

#[tokio::main]
async fn main() {
    let api = AsyncApi::new(tg_bot::TOKEN);

    Database::create_table();

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    let keyboard_markup = tg_bot::set_keyboard_markup().await;

    loop {
        let result = api.get_updates(&update_params).await;

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {

                        let api_clone = api.clone();

                        let keyboard_clone = keyboard_markup.clone();

                        tokio::spawn(async move {
                            let (send_message_params, send_photo_params) = tg_bot::send_message(message, &api_clone, &keyboard_clone).await;

                            if let Err(err) = api_clone.send_message(&send_message_params).await {
                                eprintln!("Failed to send message: {err:?}");
                            }

                            if let Err(err) = api_clone.send_photo(&send_photo_params).await {
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
