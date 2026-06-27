use crate::text_core::Editor;
use egui::{Align2, Color32, FontId, Pos2, Stroke};
use fontdue::Font;
use std::sync::OnceLock;

static FONT: OnceLock<Font> = OnceLock::new();

fn get_font() -> &'static Font {
    FONT.get_or_init(|| {
        let font_data = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
        Font::from_bytes(font_data.as_slice(), fontdue::FontSettings::default())
            .expect("Не удалось загрузить шрифт")
    })
}

fn measure_text_width(text: &str, font: &Font, size: f32) -> f32 {
    let mut width = 0.0;
    for ch in text.chars() {
        let (metrics, _) = font.rasterize(ch, size);
        width += metrics.advance_width;
    }
    width
}

pub struct IdeApp {
    editor: Editor,
}

impl IdeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            editor: Editor::new(),
        }
    }
}

impl eframe::App for IdeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.input(|i| {
            for event in &i.events {
                if let egui::Event::Text(text) = event {
                    for ch in text.chars() {
                        self.editor.insert_char(ch);
                    }
                }
            }

            if i.key_pressed(egui::Key::Backspace) {
                self.editor.delete_backspace();
            }

            if i.key_pressed(egui::Key::Enter) {
                self.editor.insert_enter();
            }

            if i.key_pressed(egui::Key::ArrowLeft) {
                self.editor.move_left();
            }

            if i.key_pressed(egui::Key::ArrowRight) {
                self.editor.move_right();
            }
        });

        let font_size = 14.0;
        let row_height = 18.0;
        let start_x = 15.0;
        let start_y = 50.0;

        let current_line = self.editor.current_line();
        let text_before_cursor = self.editor.text_before_cursor_in_line();

        let text_width = if text_before_cursor.is_empty() {
            0.0
        } else {
            measure_text_width(&text_before_cursor, get_font(), font_size)
        };

        egui::CentralPanel::default().show(ui, |ui_panel| {
            ui_panel.heading("Quartz IDE");
            ui_panel.add_space(10.0);

            let painter = ui_panel.painter();

            for (row_idx, line) in self.editor.text.lines().enumerate() {
                let y = start_y + (row_idx as f32 * row_height);
                painter.text(
                    Pos2::new(start_x, y),
                    Align2::LEFT_TOP,
                    line.to_string().trim_end_matches('\n'),
                    FontId::monospace(font_size as f32),
                    Color32::from_rgb(200, 200, 200),
                );
            }

            let cursor_x = start_x + text_width;
            let cursor_y = start_y + (current_line as f32 * row_height);
            painter.line_segment(
                [
                    Pos2::new(cursor_x, cursor_y),
                    Pos2::new(cursor_x, cursor_y + row_height),
                ],
                Stroke::new(2.0, Color32::from_rgb(100, 200, 255)),
            );
        });
    }
}
