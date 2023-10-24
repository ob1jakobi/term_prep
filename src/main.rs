use cursive::views::{SelectView, TextView};
use cursive::Cursive;
use serde::{Deserialize, Serialize};

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

fn main() {
    let mut siv = Cursive::default();

    // File for the JSON exam chosen that consists of a list of questions in a format that is able to be parsed
    // into the format of the `Question` struct above.
    let mut exam_filename: String = String::new();

    // The logo as ASCII art for display as the title of the main menu.
    let logo_view = TextView::new(LOGO);

    // Select Exam Type, based on

    // View for selecting which exam you want to study for. Exams are based on JSON files that store the questions
    // for the respective exam.
    let exam_select_view = SelectView::<String>::new();

    // RadioGroup for selecting which quiz type the user wants to conduct

    // siv.run();
}
