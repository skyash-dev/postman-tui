pub enum CurrentScreen {
    Main,
    Editing,
}

pub enum CurrentlyEditing {
    Url,
    Verb,
}

pub struct App {
    pub url_input: String,
    pub verb_input: String,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> App {
        App {
            url_input: String::new(),
            verb_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn make_request(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }
}
