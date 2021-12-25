use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Task {
    gid: String,
    name: String,
    resource_type: String,
    #[serde(skip)]
    workspace_gid: String,
}

#[derive(Deserialize, Debug)]
pub struct Tasks {
    data: Vec<Task>,
}

pub fn search_tasks(workspace_gid: &str, text: &str, pats: &str) -> Result<Tasks> {
    let json = do_search_tasks(workspace_gid, text, pats)?;
    let tasks: Tasks = serde_json::from_str(&json)?;

    Ok(tasks)
}

fn do_search_tasks(workspace_gid: &str, text: &str, pats: &str) -> Result<String> {
    let url = format!(
        "https://app.asana.com/api/1.0/workspaces/{}/tasks/search?text={}",
        workspace_gid, text
    );
    let cli = Client::new();
    let res = cli.get(url).bearer_auth(pats).send()?;
    if res.status() == StatusCode::OK {
        return Ok(res.text()?);
    }

    Err(anyhow!(
        "Failed to search tasks in a workspace app.asana.com"
    ))
}
