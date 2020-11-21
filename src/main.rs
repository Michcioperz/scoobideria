use futures::StreamExt;
use lalrpop_util::lalrpop_mod;
use telegram_bot::prelude::*;

lalrpop_mod!(pub grammar);

fn answer(query: &str) -> String {
    match grammar::QueryParser::new().parse(query) {
        Ok(v) => {
            let sum: isize = v.iter().sum();
            let msg = format!("{} {:?}", sum, v);
            if msg.len() > 4096 {
                format!("{} [too long to list]", sum)
            } else {
                msg
            }
        }
        Err(e) => format!("{}", e),
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
                    api.send(message.text_reply(answer(data))).await?;
                }
            },
            _ => { },
        }
    }
    Ok(())
}
