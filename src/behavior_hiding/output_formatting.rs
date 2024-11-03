/* B.2.3 Output Formatting (Angie) */
use colored::Colorize;

pub struct OutputFormatter {
    format_style: FormatStyle,
}

pub enum FormatStyle {
    Plain,
    Colored,
}

impl OutputFormatter {
    // Constructor for OutputFormatter
    pub fn new(format_style: FormatStyle) -> Self {
        OutputFormatter { format_style }
    }

    pub fn display_syntax_error(&self, error_message: &str) {
        let formatted_message = match self.format_style {
            FormatStyle::Plain => format!("{}", error_message),
            FormatStyle::Colored => format!("{}", error_message.red()),
        };
        eprintln!("{}", formatted_message); // Displaying error message to stderr
    }

    pub fn display_program_result(&self, program_report: &str) {
        let formatted_message = match self.format_style {
            FormatStyle::Plain => program_report.to_string(),
            FormatStyle::Colored => program_report.green().to_string(),
        };
        println!("{}", formatted_message); // Displaying result message to stdout
    }

    pub fn display_command_execution_status(&self, success: bool, command: &str) {
        let message = if success {
            format!("Command '{}' executed successfully.", command)
        } else {
            format!("Command '{}' failed to execute.", command)
        };
        self.display_program_result(&message);
    }
}
