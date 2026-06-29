use iced::widget::{container, button, column};
use iced::{Element, Task};
use iced_code_editor::{CodeEditor, Message as EditorMessage};
use iced_aw::menu::{Item, Menu, MenuBar};

pub struct MyApp {
    editor: CodeEditor,
    current_file: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditorEvent(EditorMessage),
    Open,
    Save,
    SaveAs,
    Exit,
}

impl Default for MyApp {
    fn default() -> Self {
        let code = r#"fn main() {
    println!("Hello, world!");
}
"#;
        Self {
            editor: CodeEditor::new(code, "rust"),
            current_file: None,
        }
    }
}

impl MyApp {
    fn open_file(&mut self) -> Task<Message> {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                self.editor = CodeEditor::new(&contents, "rs");
                self.current_file = Some(path);
                println!("File open: {:?}", self.current_file);
            } else {
                eprintln!("Error to read file");
            }
        }
        Task::none()
    }

    pub fn save_file(&mut self) -> Task<Message> {
        if let Some(path) = &self.current_file {
            let content = self.editor.content();
            if let Err(e) = std::fs::write(path, content) {
                eprintln!("Error save: {}", e);
            } else {
                println!("File save {:?}", path);
            }
        } else {
            self.save_as();
        }
        Task::none()
    }

    pub fn save_as(&mut self) -> Task<Message> {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            let content = self.editor.content();
            if let Err(e) = std::fs::write(&path, content) {
                eprintln!("Error to save: {}", e);
            } else {
                self.current_file = Some(path);
                println!("File save as: {:?}", self.current_file);
            }
        }
        Task::none()
    }
}

impl MyApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EditorEvent(event) => {
                self.editor.update(&event).map(Message::EditorEvent)
            }
            Message::Open => self.open_file(),
            Message::Save => self.save_file(),
            Message::SaveAs => self.save_as(),
            Message::Exit => {
                std::process::exit(0);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let open_item = Item::new(button("Open").on_press(Message::Open));
        let save_item = Item::new(button("Save").on_press(Message::Save));
        let save_as_item = Item::new(button("Save As").on_press(Message::SaveAs));
        let exit_item = Item::new(button("Exit").on_press(Message::Exit));

        let file_menu = Item::with_menu(
            button("File"),
            Menu::new(vec![open_item, save_item, save_as_item, exit_item]).width(100),
        );

        let menu_bar = MenuBar::new(vec![file_menu]);

        let editor_widget = self.editor.view().map(Message::EditorEvent);

        column![
            menu_bar,
            container(editor_widget).padding(20)
        ]
        .into()
    }
}
