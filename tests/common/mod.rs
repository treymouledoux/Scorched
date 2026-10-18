use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

// Each integration-test executable has its own global logger configuration.
// Keep one logging scenario per executable, so cargo's parallel test runner
// cannot change another scenario's threshold or one-time path/prefix settings.
pub struct Logs {
    pub path: PathBuf,
}

impl Logs {
    pub fn new(name: &str, configure: impl FnOnce(&str)) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("scorched-{name}-{}-{nonce}", std::process::id()));
        assert!(!path.exists());
        configure(path.to_str().unwrap());
        Self { path }
    }

    pub fn read(&self) -> String {
        let mut contents = String::new();
        let entries = fs::read_dir(&self.path).expect("logging must create its directory");
        for entry in entries {
            let path = entry.unwrap().path();
            assert_eq!(path.extension().unwrap(), "log");
            let date = path.file_stem().unwrap().to_str().unwrap();
            let text = fs::read_to_string(&path).expect("log file must be readable");
            for line in text.lines() {
                assert!(
                    line.starts_with(date),
                    "timestamp must match the log file's UTC date"
                );
            }
            contents.push_str(&text);
        }
        assert!(
            !contents.is_empty(),
            "logging must write at least one record"
        );
        contents
    }
}

impl Drop for Logs {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
