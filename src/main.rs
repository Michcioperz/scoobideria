use futures::StreamExt;
use lalrpop_util::lalrpop_mod;
use telegram_bot::prelude::*;

lalrpop_mod!(pub grammar);

pub fn answer(query: &str) -> Result<String, String> {
    match grammar::QueryParser::new().parse(query) {
        Ok(v) => {
            let sum: isize = v.iter().sum();
            let msg = format!("{} {:?}", sum, v);
            if msg.len() > 4096 {
                Ok(format!("{} [too long to list]", sum))
            } else {
                Ok(msg)
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN missing");
    let api = telegram_bot::Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        match update?.kind {
            telegram_bot::UpdateKind::Message(message) => {
                if let telegram_bot::MessageKind::Text { ref data, .. } = message.kind {
                    if let Ok(s) = answer(data) {
                        api.send(message.text_reply(s)).await?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
