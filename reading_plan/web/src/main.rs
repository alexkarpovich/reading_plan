use std::sync::Arc;

use shared::app::services::bible_service::BibleService;
use shared::app::gateways::abstract_bible::BibleGateway;
use shared::app::gateways::abstract_parsing::ParseReference;
use shared::adapters::gateways::parse_gateway::ParsingGateway;
use shared::adapters::gateways::postgres::bible_gateway::PostgresBibleGateway;
use shared::adapters::persistence::postgres::create_pool;
use shared::app::usecases::bible::GetVersesByRef;

#[tokio::main]
async fn main() {
    println!("Reading plang web is starting...");
    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(err) => panic!("DB init error")
    };
    let bible_service = BibleService::new(
        Arc::new(PostgresBibleGateway::new(pool)),
        Arc::new(ParsingGateway{}),
    );

    let verses = match bible_service.list_verses_by_ref("Кор 1:3", "ru").await {
        Ok(verses) => verses,
        Err(err) => panic!("Get verses error")
    };
    println!("{:?}", verses);
}
