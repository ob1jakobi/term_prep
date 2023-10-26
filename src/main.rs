use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::{Cursive, CursiveExt};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

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
    has_seen: bool,
    answered_incorrectly: bool,
}

struct _User {
    username: String,
    exams: Vec<String>,
    seen_questions: Vec<Question>,
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
        match s.call_on_name("filename_entry", |v: &mut EditView| v.get_content()) {
            Some(filename) => validate_exam_filename(s, &filename),
            _ => validate_exam_filename(s, ""),
        }
    })
    .button("Quit", |s: &mut Cursive| {
        s.quit();
    });

    siv.add_layer(start_dialog);

    siv.run();
}

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

fn select_test_type(s: &mut Cursive, _f: File) {
    s.add_layer(Dialog::info("Yay! You've validated a file"));
    thread::sleep(Duration::from_secs(2));
    s.quit();
}
