use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct SearchTasksData {
    gid: String,
    pub name: String,
    #[allow(dead_code)]
    resource_type: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchTasks {
    pub data: Vec<SearchTasksData>,
}

impl SearchTasksData {
    pub fn get_permalink_url(&self, pats: &str) -> Result<String> {
        let json = self.do_get_permalink_url(pats)?;
        let root: Value = serde_json::from_str(&json)?;
        root.get("data")
            .and_then(|v| v.get("permalink_url"))
            .and_then(|v| v.as_str())
            .map(|v| format!("{}/f", v))
            .ok_or(anyhow!("Failed to extract permalink_url"))
    }

    fn do_get_permalink_url(&self, pats: &str) -> Result<String> {
        // NOTE: https://developers.asana.com/docs/get-a-task
        let url = format!("https://app.asana.com/api/1.0/tasks/{}", self.gid);
        let cli = Client::new();
        // NOTE: https://developers.asana.com/docs/personal-access-token
        let res = cli.get(url).bearer_auth(pats).send()?;
        if res.status() != StatusCode::OK {
            return Err(anyhow!("Failed to get task in a workspace app.asana.com"));
        }

        Ok(res.text()?)
    }
}

pub fn search_tasks(workspace_gid: &str, text: &str, pats: &str) -> Result<SearchTasks> {
    let json = do_search_tasks(workspace_gid, text, pats)?;
    let tasks: SearchTasks = serde_json::from_str(&json)?;

    Ok(tasks)
}

fn do_search_tasks(workspace_gid: &str, text: &str, pats: &str) -> Result<String> {
    // NOTE: https://developers.asana.com/docs/search-tasks-in-a-workspace
    let url = format!(
        "https://app.asana.com/api/1.0/workspaces/{}/tasks/search?text={}",
        workspace_gid, text
    );
    let cli = Client::new();
    // NOTE: https://developers.asana.com/docs/personal-access-token
    let res = cli.get(url).bearer_auth(pats).send()?;
    if res.status() != StatusCode::OK {
        return Err(anyhow!(
            "Failed to search tasks in a workspace app.asana.com"
        ));
    }

    Ok(res.text()?)
}

pub fn get_workspace(workspace_gid: &str, pats: &str) -> Result<bool> {
    // NOTE: https://developers.asana.com/docs/get-a-workspace
    let url = format!("https://app.asana.com/api/1.0/workspaces/{}", workspace_gid);
    let cli = Client::new();
    let res = cli.get(url).bearer_auth(pats).send()?;

    if res.status() != StatusCode::OK {
        return Err(anyhow!("Failed to access me"));
    }

    Ok(true)
}
