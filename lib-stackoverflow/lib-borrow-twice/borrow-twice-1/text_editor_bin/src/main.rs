use text_editor_lib::TextEditor;

fn main() {
    let mut editor = TextEditor::new("Initial content");

    editor.edit("First change");
    editor.print_content();

    editor.edit("Second change");
    editor.print_content();

    if editor.undo().is_ok() {
        editor.print_content();
    }
}
