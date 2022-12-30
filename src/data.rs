use anyhow::{Context, Result};
use reqwest::{blocking::Client, StatusCode};
use serde::Serialize;
use things3::{list::List, todo::Todo};

use crate::config::Config;

#[derive(Debug, Serialize, PartialEq)]
pub struct Data {
    pub today_todos: Vec<Todo>,
}

impl Data {
    pub fn send(&self, client: &Client, config: &Config) -> Result<()> {
        let resp = client
            .post(&config.endpoint)
            .bearer_auth(&config.bearer_token)
            .json(self)
            .send()
            .context("failed to send new data")?;
        anyhow::ensure!(
            resp.status() == StatusCode::OK,
            "response didn't have status code 200, failed to send new data"
        );
        Ok(())
    }

    pub fn fetch() -> Result<Self> {
        Ok(Self {
            today_todos: List::Today.fetch_todos()?,
        })
    }
}
