mod gui;

fn main() -> iced::Result {
    iced::run(gui::MyApp::update, gui::MyApp::view)
}


