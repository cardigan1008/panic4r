use std::cell::RefCell;
use std::collections::VecDeque;

pub struct TextEditor {
    content: String,
    history: RefCell<VecDeque<String>>,
}

impl TextEditor {
    pub fn new(initial_content: &str) -> TextEditor {
        TextEditor {
            content: initial_content.to_string(),
            history: RefCell::new(VecDeque::new()),
        }
    }

    pub fn edit(&mut self, new_content: &str) {
        self.history.borrow_mut().push_front(self.content.clone());
        self.content = new_content.to_string();
    }

    pub fn undo(&mut self) -> Result<(), String> {
        let mut history_borrow = self.history.borrow_mut();
        if let Some(previous_content) = history_borrow.pop_front() {
            self.content = previous_content;
            self.history.borrow(); 
            Ok(())
        } else {
            Err("No previous history available.".to_string())
        }
    }

    pub fn print_content(&self) {
        println!("Current content: {}", self.content);
    }
}
