use cursive::views::{Dialog, TextView};

const LOGO: &str = "
 ______   ______     ______     __    __        ______   ______     ______     ______  
/\__  _\ /\  ___\   /\  == \   /\ \"-./  \      /\  == \ /\  == \   /\  ___\   /\  == \ 
\/_/\ \/ \ \  __\   \ \  __<   \ \ \-./\ \     \ \  _-/ \ \  __<   \ \  __\   \ \  _-/ 
   \ \_\  \ \_____\  \ \_\ \_\  \ \_\ \ \_\     \ \_\    \ \_\ \_\  \ \_____\  \ \_\   
    \/_/   \/_____/   \/_/ /_/   \/_/  \/_/      \/_/     \/_/ /_/   \/_____/   \/_/   
                                                                                       
";

fn main() {
    let mut siv = Cursive::default();
    
    // The logo as ASCII art.
    let logo_view = TextView::new(LOGO);

    // View for selecting which exam you want to study for. Exams are based on JSON files that store the questions
    // for the respective exam.
    let exam_select_view;
}
