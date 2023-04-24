pub fn main(){
    
}

pub fn format_name(program: &str) -> String{
    let mut title = String::new();
    for (idx, word) in program.split('-').enumerate(){
        if word == "ai"{
            title.push_str(&word.to_uppercase());
            title.push(' ');
            continue;
        }
        title.push_str(&word[0..1].to_uppercase());
        title.push_str(&word[1..]);
        title.push(' ');
    }
    title.pop();
    title
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_assistant_works() {
        let result = format_name(&"ai-assistant");
        assert_eq!(result, String::from("AI Assistant"));
    }
    fn calculator_works() {
        let result = format_name(&"calculator");
        assert_eq!(result, String::from("Calculator"));
    }
    fn development_timer_works() {
        let result = format_name(&"development-timer");
        assert_eq!(result, String::from("Development Timer"));
    }
    fn file_organizer_works() {
        let result = format_name(&"file-organizer");
        assert_eq!(result, String::from("File Organizer"));
    }
    fn password_generator_works() {
        let result = format_name(&"password-generator");
        assert_eq!(result, String::from("Password Generator"));
    }
    fn password_manager_works() {
        let result = format_name(&"password-manager");
        assert_eq!(result, String::from("Password Manager"));
    }
    fn task_manager_works() {
        let result = format_name(&"task-manager");
        assert_eq!(result, String::from("Task Manager"));
    }
    fn word_analyzer_works() {
        let result = format_name(&"word-analyzer");
        assert_eq!(result, String::from("Word analyzer"));
    }
}

// "file-organizer",
// "password-manager", // done, needs Polish
// "password-generator", // done, needs Polish
// "task-manager", 
// "word analyzer",