pub struct TextEditor {
    content: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            content: String::new(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn save_state(&self) -> EditorMemento {
        EditorMemento::new(self.content.clone())
    }

    pub fn restore_state(&mut self, memento: EditorMemento) {
        self.content = memento.content;
    }
}

pub struct EditorMemento {
    content: String,
}

impl EditorMemento {
    fn new(content: String) -> Self {
        EditorMemento { content }
    }
}

// Caretaker
pub struct EditorCaretaker {
    history: Vec<EditorMemento>,
}

impl EditorCaretaker {
    pub fn new() -> Self {
        EditorCaretaker { history: vec![] }
    }

    pub fn save(&mut self, editor: &TextEditor) {
        self.history.push(editor.save_state());
    }

    pub fn undo(&mut self, editor: &mut TextEditor) -> Result<(), &str> {
        match self.history.pop() {
            Some(memento) => {
                editor.restore_state(memento);
                Ok(())
            }
            None => Err("No history available to undo"),
        }
    }
}
