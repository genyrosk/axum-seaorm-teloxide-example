// fn main() {
//     api::main();
// }

#[tokio::main]
async fn main() {
    // let api_server = tokio::spawn(api::start());
    // bot::hello_world().await;

    let mut botto = bot::telegram_bot_dispatch();

    tokio::join!(api::start(), bot::hello_world(), botto.dispatch());
}
