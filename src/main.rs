use config::Config;

mod config;

fn main() {
    let config = Config::parse(Config::read()).expect("parsing configuration file failed");
    dbg!(config);
}
