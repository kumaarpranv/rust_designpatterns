use crate::behavioural::memento::{TextEditor, EditorCaretaker};

#[test]
fn editor_test() {
    let mut editor = TextEditor::new();
    let mut caretaker = EditorCaretaker::new();

    editor.set_content("hello");
    caretaker.save(&editor);

    editor.set_content("world.");
    caretaker.save(&editor);

    editor.set_content("this is me.");

    // Undo last change
    caretaker.undo(&mut editor).unwrap();
    println!("Content after undo: {}", editor.content());
    assert_eq!("world.".to_string(), editor.content());
    // Undo one more change
    caretaker.undo(&mut editor).unwrap();
    println!("Content after another undo: {}", editor.content());
    assert_eq!("hello".to_string(), editor.content());
}
