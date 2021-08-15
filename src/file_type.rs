pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighlightingOptions {
    numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn from(filename: &str) -> Self {
        if filename.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        }
        Self::default()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn highlighting_options(&self) -> HighlightingOptions {
        self.hl_opts
    }
}

impl HighlightingOptions {
    pub fn numbers(self) -> bool {
        self.numbers
    }
}
