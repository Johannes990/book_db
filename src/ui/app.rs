pub enum Screen {
    DataBaseEntryView,
}

pub enum PopUp {
    None,
    QuitDialog,
}

pub struct App {
    pub current_screen: Screen,
    pub current_popup: PopUp,
}

impl App {
    pub fn new() -> Self { 
        Self {
            current_screen: Screen::DataBaseEntryView,
            current_popup: PopUp::None,
        }
    }
}