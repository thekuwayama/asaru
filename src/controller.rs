use anyhow::Result;

use crate::asana;

pub struct State {
    pub text: String,
    pub tasks: Vec<asana::SearchTasksData>,
    workspace_gid: String,
    pub pats: String,
    pub index: usize,
}

impl State {
    pub fn new(workspace_gid: &str, pats: &str) -> Self {
        State {
            text: String::new(),
            tasks: Vec::new(),
            workspace_gid: workspace_gid.to_string(),
            pats: pats.to_string(),
            index: 0,
        }
    }

    pub fn search(&mut self) -> Result<&mut Self> {
        self.tasks = asana::search_tasks(&self.workspace_gid, &self.text, &self.pats)?.data;

        Ok(self)
    }

    pub fn get_permalink_urls(&self, indexes: &[usize]) -> Vec<String> {
        indexes
            .iter()
            .map(|i| self.tasks.get(*i))
            .flat_map(|r| r)
            .map(|t| t.get_permalink_url(&self.pats))
            .flat_map(|r| r)
            .collect::<Vec<_>>()
    }

    pub fn get_titles(&self) -> Vec<String> {
        self.tasks
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<_>>()
    }
}
