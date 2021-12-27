use std::collections::HashSet;

use anyhow::Result;

use crate::asana;

pub struct State {
    pub text: String,
    pub tasks: Vec<asana::SearchTasksData>,
    workspace_gid: String,
    pub pats: String,
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

    pub fn search(&mut self) -> Result<&mut Self> {
        self.tasks = asana::search_tasks(&self.workspace_gid, &self.text, &self.pats)?.data;

        Ok(self)
    }

    pub fn get_permalink_urls(&self, indexes: &[usize]) -> Vec<String> {
        indexes
            .iter()
            .filter_map(|i| self.tasks.get(*i))
            .filter_map(|t| t.get_permalink_url(&self.pats).ok())
            .collect::<Vec<_>>()
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

    pub fn check(&mut self) -> Option<usize> {
        if self.tasks.is_empty() {
            return None;
        }

        self.checked.insert(self.index);

        Some(self.index)
    }

    pub fn get_checked_permalink_urls(&self) -> Vec<String> {
        self.checked
            .iter()
            .filter_map(|i| self.tasks.get(*i))
            .filter_map(|t| t.get_permalink_url(&self.pats).ok())
            .collect::<Vec<_>>()
    }
}
