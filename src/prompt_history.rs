use ncurses::*;
use std::cmp::min;

static PROMPT_HISTORY_SIZE: usize = 5;

pub struct PromptHistory {
    content: Vec<String>,
}

pub fn make() -> PromptHistory {
    PromptHistory {
        content: Vec::new(),
    }
}

impl PromptHistory {
    pub fn update(&mut self, content: String) {
        self.content.push(content);
        if self.content.len() > PROMPT_HISTORY_SIZE {
            self.content.remove(0);
        }
    }

    pub fn display(&self) {
        let len = self.content.len();
        let start_index = len - min(len, PROMPT_HISTORY_SIZE);
        for item in &self.content[start_index..len] {
            addstr(&item);
        }
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }
}
