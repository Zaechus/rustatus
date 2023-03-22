pub struct Config {
    red: String,
    green: String,
    yellow: String,
}

impl Config {
    pub fn new<S: Into<String>>(red: S, green: S, yellow: S) -> Self {
        Self {
            red: red.into(),
            green: green.into(),
            yellow: yellow.into(),
        }
    }

    pub fn red(&self) -> &str {
        &self.red
    }

    pub fn green(&self) -> &str {
        &self.green
    }

    pub fn yellow(&self) -> &str {
        &self.yellow
    }
}
