use crate::options::OptionKind;

pub struct SelectableField {
    pub kind: OptionKind,
    pub selected: bool,
}

impl SelectableField {
    pub fn toggle(&mut self) {
        if let OptionKind::Toggle(v) = &mut self.kind {
            *v = !*v;
        }
    }

    pub fn enter_char(&mut self, c: char) {
        if let OptionKind::TextInput(ref mut val) = self.kind {
            val.push(c);
        }
    }

    pub fn pop_char(&mut self) {
        if let OptionKind::TextInput(ref mut val) = self.kind {
            val.pop();
        }
    }

    pub fn parse_value<T: std::str::FromStr>(&self) -> Option<T> {
        if let OptionKind::TextInput(val) = &self.kind {
            val.parse::<T>().ok()
        } else {
            None
        }
    }
}
