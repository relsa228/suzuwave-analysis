pub const BLOCK_TITLE: &str = "Command Table";
pub const COMMAND_LIST_TABLE_HEADERS: (&str, &str, &str) = ("Command", "Arguments", "Description");
pub const COMMAND_LIST: [(&str, &str, &str); 18] = [
    //GeneralCommands
    ("ce", "-", "Show or hide chart explorer"),
    ("a", "-", "Show application about information"),
    ("h", "-", "Show commands table"),
    ("q", "-", "Quit application"),
    //ChartViewCommands
    ("zi", "Scale coefficient (Float)", "Enlarge chart"),
    ("zo", "Scale coefficient (Float)", "Shrink chart"),
    ("ml", "Number of steps (Int)", "Move chart left"),
    ("mr", "Number of steps (Int)", "Move chart right"),
    ("fft", "-", "Perform Fast Fourier Transform"),
    (
        "sft",
        "window size (Int), hop size (Int)",
        "Perform Short-Time Fourier Transforms",
    ),
    ("hwt", "-", "Perform Haar Wavelet Transform"),
    ("flp", "filter value (Float)", "Apply LowPass Filter"),
    ("fhp", "filter value (Float)", "Apply HighPass Filter"),
    (
        "fbp",
        "low filter value (Float) high filter value (Float)",
        "Apply BandPass Filter",
    ),
    (
        "fbs",
        "low filter value (Float) high filter value (Float)",
        "Apply BandStop Filter",
    ),
    //ChartExplorerCommands
    ("of", "File path (String)", "Open new signal file"),
    ("cwv", "-", "Close current chart view"),
    ("swv", "View index (Int)", "Move to the another chart view"),
];
