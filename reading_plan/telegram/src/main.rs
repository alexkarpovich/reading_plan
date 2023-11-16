use std::env;
use url::Url;
use teloxide::{
    dispatching::dialogue::InMemStorage, 
    prelude::*, 
    types::{
        InlineKeyboardMarkup, 
        InlineKeyboardButton, 
        InlineKeyboardButtonKind,
        WebAppInfo
    }
};


type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_TOKEN")
        .expect("TELEGRAM_TOKEN was not provided.");
    let bot = Bot::new(token);

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let mut markup = InlineKeyboardMarkup::default();
    let buttons = vec![
        InlineKeyboardButton::new(
            "Open web app",
            InlineKeyboardButtonKind::WebApp(
                WebAppInfo { url: Url::parse("https://bibleplan.ru")? }
            )
        )
    ];
    markup = markup.append_row(buttons);
    
    bot.send_message(msg.chat.id, "Let's start! What's your full name?")
        .reply_markup(markup)
        .send()
        .await?;
    
    dialogue.update(State::ReceiveFullName).await?;
    
    Ok(())
}
