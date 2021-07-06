use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");
    println!("asdfasdf");

    // let bot = Bot::new("1").auto_send();

    // bot.send_message(1, "start").send().await;

    teloxide::repl(bot, |message| async move {
        dbg!(message.update.chat.id);
        message.answer_dice().await?;
        respond(())
    })
        .await;
}