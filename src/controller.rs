use std::collections::HashSet;

use anyhow::Result;
use futures::future;

use crate::asana;

pub(crate) struct State {
    workspace_gid: String,
    pats: String,
    text: String,
    tasks: Vec<asana::SearchTasksData>,
    index: usize,
    checked: HashSet<usize>,
}

impl State {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tasks(&self) -> &[asana::SearchTasksData] {
        &self.tasks
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn checked(&self) -> &HashSet<usize> {
        &self.checked
    }

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

    pub fn clear_checked(mut self) -> Self {
        self.checked = HashSet::new();
        self
    }

    pub fn clear_index(mut self) -> Self {
        self.index = 0;
        self
    }

    pub fn dec_index(mut self) -> Self {
        self.index -= 1;
        self
    }

    pub fn inc_index(mut self) -> Self {
        self.index += 1;
        self
    }

    pub fn edit_index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn edit_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub async fn search(mut self) -> Result<Self> {
        let tasks = asana::search_tasks(&self.workspace_gid, &self.text, &self.pats)
            .await?
            .data;
        self.tasks = tasks;
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

    pub fn check(mut self) -> Self {
        if self.tasks.len() > self.index {
            let mut hs = self.checked;
            hs.insert(self.index);
            self.checked = hs;
            return self;
        }

        self
    }

    pub fn uncheck(mut self) -> Self {
        let mut hs = self.checked;
        hs.remove(&self.index);
        self.checked = hs;
        self
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
