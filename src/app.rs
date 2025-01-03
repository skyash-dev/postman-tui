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
    pub response_data: Option<String>,
}

impl App {
    pub fn new() -> App {
        App {
            url_input: String::new(),
            verb_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            response_data: None,
        }
    }

    pub fn make_request(&mut self) {
        self.response_data = Some(format!(
            "Making request to {} with verb {}",
            self.url_input, self.verb_input
        ));

        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }
}
