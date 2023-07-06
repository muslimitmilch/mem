mod editor;
mod terminal;
mod document;

use editor::Editor;
pub use terminal::Terminal;
pub use document::Document;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
