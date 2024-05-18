use std::env;
use clickhouse::{ Client, Row};
use dotenv::dotenv;
use time::Date;
use serde::{Deserialize, Serialize};


#[derive(Debug, Row, Serialize, Deserialize)]
struct Data{
    #[serde(with = "clickhouse::serde::time::date")]
    date: Date,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64
}

async fn parse_data(date: String) -> anyhow::Result<Data> {
    dotenv().ok();
    let password = env::var("PASSWORD").expect("Cannot find password in .env file");
    let client = Client::default()
        .with_url("https://famep8kcv5.ap-southeast-1.aws.clickhouse.cloud:8443")
        .with_user("default")
        .with_password(password)
        .with_database("default");

    let data = fetch(&client, date).await?;

    Ok(data)
}


async fn fetch(client: &Client, date: String) -> anyhow::Result<Data> {
    let mut cursor = client
        .query("select * from TLT3 where Date == ?")
        .bind(date)
        .fetch::<Data>()?;

    let Some(date) = cursor.next().await? else {panic!("db error")};

    Ok(date)
}


#[tokio::main]
async fn main() {
    let start_price = parse_data("2024-01-01".to_string()).await?;
    println!("{:?}", start_price)
}
