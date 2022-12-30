use std::{thread::sleep, time::Duration};

use config::Config;
use data::Data;
use reqwest::blocking::Client;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod data;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("booted");
    let config = Config::parse(Config::read()).expect("parsing configuration file failed");
    info!("loaded config");
    let client = Client::new();

    let mut recent_data = Data::fetch().expect("failed to initially fetch data");
    info!("obtained initial data");
    recent_data
        .send(&client, &config)
        .expect("failed to send initial data to endpoint");
    dbg!(&recent_data);
    info!("send initial data");
    let wait_time = Duration::from_secs(config.refresh as u64);

    println!();
    loop {
        recent_data = Data::fetch().expect("failed to fetch most recent data");
        recent_data
            .send(&client, &config)
            .expect("failed to send newest data");
        info!("sent data");
        sleep(wait_time);
    }
}
