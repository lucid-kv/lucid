use colored::*;

enum LucidError
{
    Information,
    Warning,
    Error
}

fn display_error(status: LucidError, message: String)
{
    println!("{} {}", match status
    {
        LucidError::Information => {
            "Information:".bright_green()
        },
        LucidError::Warning => {
            "Warning:".bright_yellow()
        },
        LucidError::Error => {
            "Error:".bright_red()
        }
    }, message);
}