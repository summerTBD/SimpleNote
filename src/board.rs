use crate::note::Note;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Board {
    notes: Vec<Note>,
    next_id: u64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            notes: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_note(&mut self, title: String, content: String) -> &Note {
        let note = Note::new(self.next_id, title, content);
        self.next_id += 1;
        self.notes.push(note);
        self.notes.last().unwrap()
    }

    pub fn delete_note(&mut self, id: u64) -> Result<(), String> {
        let pos = self.notes.iter().position(|n| n.id == id);
        match pos {
            Some(idx) => {
                self.notes.remove(idx);
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {} 的便利贴", id)),
        }
    }

    pub fn edit_note(&mut self, id: u64, title: String, content: String) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.title = title;
                note.content = content;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {} 的便利贴", id)),
        }
    }

    pub fn hide_note(&mut self, id: u64) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.hidden = true;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {} 的便利贴", id)),
        }
    }

    pub fn unhide_note(&mut self, id: u64) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.hidden = false;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {} 的便利贴", id)),
        }
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }
}
