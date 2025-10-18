pub const BLOCK_TITLE: &str = "Command Table";
pub const COMMAND_LIST_TABLE_HEADERS: (&str, &str, &str) = ("Command", "Arguments", "Description");
pub const COMMAND_LIST: [(&str, &str, &str); 11] = [
    ("fe", "-", "Show or hide file explorer"),
    ("a", "-", "Show application about information"),
    ("h", "-", "Show commands table"),
    ("q", "-", "Quit application"),
    ("of", "File path (String)", "Open new signal flag"),
    ("zi", "Scale coefficient (Float)", "Enlarge chart"),
    ("zo", "Scale coefficient (Float)", "Shrink chart"),
    ("ml", "Number of steps (Int)", "Move chart left"),
    ("mr", "Number of steps (Int)", "Move chart right"),
    ("cwv", "-", "Close current chart view"),
    ("swv", "View index (Int)", "Move to the another chart view"),
];
