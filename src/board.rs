use crate::note::Note;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Board {
    notes: Vec<Note>,
    next_id: u64,
}

impl Board {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_note(&mut self, title: String, content: String) -> &Note {
        let note = Note::new(self.next_id, title, content);
        self.next_id += 1;
        self.notes.push(note);
        self.notes.last().expect("a note was just pushed")
    }

    /// Delete a note by its id.
    ///
    /// # Errors
    ///
    /// Returns an error if no note with the given id exists.
    pub fn delete_note(&mut self, id: u64) -> Result<(), String> {
        let pos = self.notes.iter().position(|n| n.id == id);
        match pos {
            Some(idx) => {
                self.notes.remove(idx);
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {id} 的便利贴")),
        }
    }

    /// Edit a note's title and content.
    ///
    /// # Errors
    ///
    /// Returns an error if no note with the given id exists.
    pub fn edit_note(&mut self, id: u64, title: String, content: String) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.title = title;
                note.content = content;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {id} 的便利贴")),
        }
    }

    /// Hide a note.
    ///
    /// # Errors
    ///
    /// Returns an error if no note with the given id exists.
    pub fn hide_note(&mut self, id: u64) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.hidden = true;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {id} 的便利贴")),
        }
    }

    /// Unhide a note.
    ///
    /// # Errors
    ///
    /// Returns an error if no note with the given id exists.
    pub fn unhide_note(&mut self, id: u64) -> Result<(), String> {
        match self.notes.iter_mut().find(|n| n.id == id) {
            Some(note) => {
                note.hidden = false;
                Ok(())
            }
            None => Err(format!("没有找到 id 为 {id} 的便利贴")),
        }
    }

    /// Toggle a note's hidden state.
    pub fn toggle_hide_note(&mut self, id: u64) {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.hidden = !note.hidden;
        }
    }

    /// 找到下一个可见便签的 ID（循环查找，无内存分配）
    pub fn next_visible_note(&self, current_id: Option<u64>, show_hidden: bool) -> Option<u64> {
        if self.notes.is_empty() {
            return None;
        }
        let visible = |n: &&Note| show_hidden || !n.hidden;
        let pos = current_id.and_then(|id| self.notes.iter().position(|n| n.id == id));

        // 从当前位置之后找
        let start = pos.map_or(0, |p| p + 1);
        if let Some(note) = self.notes[start..].iter().find(visible) {
            return Some(note.id);
        }
        // 没找到就从头绕回
        self.notes[..start].iter().find(visible).map(|n| n.id)
    }

    /// 找到上一个可见便签的 ID（循环查找，无内存分配）
    pub fn prev_visible_note(&self, current_id: Option<u64>, show_hidden: bool) -> Option<u64> {
        if self.notes.is_empty() {
            return None;
        }
        let visible = |n: &&Note| show_hidden || !n.hidden;
        let pos = current_id.and_then(|id| self.notes.iter().position(|n| n.id == id));

        // 从当前位置之前逆向找
        let end = pos.unwrap_or(self.notes.len());
        if let Some(note) = self.notes[..end].iter().rev().find(visible) {
            return Some(note.id);
        }
        // 没找到就从尾绕回
        self.notes[end..].iter().rev().find(visible).map(|n| n.id)
    }

    pub fn notes(&self) -> &[Note] {
        &self.notes
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
