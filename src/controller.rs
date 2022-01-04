use std::collections::HashSet;

use anyhow::Result;
use futures::future;

use crate::asana;

pub struct State {
    workspace_gid: String,
    pats: String,

    pub text: String,
    pub tasks: Vec<asana::SearchTasksData>,
    pub index: usize,
    pub checked: HashSet<usize>,
}

impl State {
    pub fn new(workspace_gid: &str, pats: &str) -> Self {
        State {
            text: String::new(),
            tasks: Vec::new(),
            workspace_gid: workspace_gid.to_string(),
            pats: pats.to_string(),
            index: 0,
            checked: HashSet::new(),
        }
    }

    pub async fn search(&mut self) -> Result<&mut Self> {
        self.tasks = asana::search_tasks(&self.workspace_gid, &self.text, &self.pats)
            .await?
            .data;

        Ok(self)
    }

    pub async fn get_permalink_url(&self) -> Option<String> {
        match self.tasks.get(self.index) {
            Some(t) => t.get_permalink_url(&self.pats).await.ok(),
            None => None,
        }
    }

    pub fn get_titles(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
    }

    pub fn is_checked(&self, index: &usize) -> bool {
        self.checked.contains(index)
    }

    pub fn check(&mut self) {
        if self.tasks.len() > self.index {
            self.checked.insert(self.index);
        }
    }

    pub fn uncheck(&mut self) {
        self.checked.remove(&self.index);
    }

    pub async fn get_checked_permalink_urls(&self) -> Vec<String> {
        let res = future::join_all({
            self.checked
                .iter()
                .flat_map(|&i| self.tasks.get(i))
                .map(|t| async move { t.get_permalink_url(&self.pats).await })
        })
        .await;

        res.iter()
            .flat_map(|r| r.as_ref().ok())
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}
