use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::{Cursive, CursiveExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

const LOGO: &str = "
                                                                        
████████╗███████╗██████╗ ███╗   ███╗    ██████╗ ██████╗ ███████╗██████╗ 
╚══██╔══╝██╔════╝██╔══██╗████╗ ████║    ██╔══██╗██╔══██╗██╔════╝██╔══██╗
   ██║   █████╗  ██████╔╝██╔████╔██║    ██████╔╝██████╔╝█████╗  ██████╔╝
   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║    ██╔═══╝ ██╔══██╗██╔══╝  ██╔═══╝ 
   ██║   ███████╗██║  ██║██║ ╚═╝ ██║    ██║     ██║  ██║███████╗██║     
   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝    ╚═╝     ╚═╝  ╚═╝╚══════╝╚═╝     
                                                                        
";

/// Parses a JSON file that's a list of questions with various data into a question that can be used in the TUI.
#[derive(Serialize, Deserialize)]
struct Question {
    prompt: String,
    choices: Vec<String>,
    answer: String,
    subject_domain: String,
}

struct _User {
    username: String,
    exams: Vec<String>,
    seen_questions: HashMap<String, Vec<Question>>,
    incorrectly_answered: HashMap<String, Vec<Question>>,
}

fn main() {
    let mut siv = Cursive::default();

    // The logo as ASCII art for display as the title of the main menu.
    let logo_view = TextView::new(LOGO);

    // EditView for getting Exam Type based on filename/location, based on file path that user enters
    let exam_name_view = EditView::new()
        .on_submit(validate_exam_filename)
        .with_name("filename_entry")
        .fixed_width(20);

    let logo_and_exam_view = LinearLayout::horizontal()
        .child(TextView::new("Enter filename:"))
        .child(DummyView)
        .child(exam_name_view);

    let start_dialog = Dialog::around(
        LinearLayout::vertical()
            .child(logo_view)
            .child(logo_and_exam_view),
    )
    .button("OK", |s| {
        if let Some(filename) = s.call_on_name("filename_entry", |v: &mut EditView| v.get_content())
        {
            s.add_layer(Dialog::info(format!("Obtained filename: {}", filename)));
            validate_exam_filename(s, &filename);
        } else {
            s.add_layer(Dialog::info("Please enter a valid filename..."));
        }
    })
    .button("Quit", |s: &mut Cursive| {
        s.quit();
    });

    siv.add_layer(start_dialog);

    siv.run();
}

/// Validates the filename the user entered in the EditView. Should do the following:
/// * Check that the file is a JSON file containing the questions formatted in a way that can be
///   parsed according to the `Question` struct above.
/// * Check to see that the user has a file that records their statistics regarding `Question`s the
///   user has seen and which ones they have answered incorrectly. If no file is available, then
///   it should create one in the `assets` directory.
/// * Assuming the previous two points have been achieved, this function will go to the next
///   layer where it asks the user what type of test they want to do.
fn validate_exam_filename(s: &mut Cursive, filename: &str) {
    if filename.is_empty() {
        s.add_layer(Dialog::info("Filename for exam must not be empty!"));
    } else if let Ok(exam_file) = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .append(false)
        .open(PathBuf::from(filename))
    {
        select_test_type(s, exam_file);
    } else {
        s.add_layer(Dialog::info(
            "Please enter a valid JSON exam file to study...",
        ));
        s.pop_layer();
    }
}

fn select_test_type(s: &mut Cursive, f: File) {
    match f.metadata() {
        Ok(md) => s.add_layer(Dialog::info(format!("File metadata:\t{:?}", md))),
        Err(e) => s.add_layer(Dialog::info(format!(
            "Error getting metadata for file: {}",
            e
        ))),
    }
}
