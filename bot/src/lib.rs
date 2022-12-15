use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use teloxide::dispatching::DefaultKey;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported with languages:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "demo command")]
    Demo(String),
}
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
    println!("new message from chat [{}]: {:?}, cmd:", msg.chat.id, cmd);
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Demo(text) => {
            bot.send_message(msg.chat.id, format!("Your text: '{text}'."))
                .await?
        }
    };
    Ok(())
}

pub fn telegram_bot_dispatch() -> Dispatcher<Bot, anyhow::Error, DefaultKey> {
    let teloxide_token =
        std::env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN not found in environment");

    let bot = Bot::new(teloxide_token);
    let handler = Update::filter_message().branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(handle_command),
    );

    let bot_dispatcher = Dispatcher::builder(bot.clone(), handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![])
        .enable_ctrlc_handler()
        .build();
    bot_dispatcher
    // bot_dispatcher.dispatch().await
}

pub async fn hello_world() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(handler));
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
