pub mod terminal;
pub mod application;

pub fn format_name(program: &str) -> String{
    let mut title = String::new();
    for (_, word) in program.split('-').enumerate(){
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
    fn format_name_test() {
        assert_eq!(format_name(&"ai-assistant"), String::from("AI Assistant"));
        assert_eq!(format_name(&"calculator"), String::from("Calculator"));
        assert_eq!(format_name(&"development-timer"), String::from("Development Timer"));
        assert_eq!(format_name(&"file-organizer"), String::from("File Organizer"));
        assert_eq!(format_name(&"password-generator"), String::from("Password Generator"));
        assert_eq!(format_name(&"password-manager"), String::from("Password Manager"));
        assert_eq!(format_name(&"task-manager"), String::from("Task Manager"));
        assert_eq!(format_name(&"word-analyzer"), String::from("Word analyzer"));
    }
}