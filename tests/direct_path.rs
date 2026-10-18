mod common;

#[test]
fn directly_initialized_path_does_not_need_a_trailing_separator() {
    let logs = common::Logs::new("direct-path", |path| {
        scorched::LOG_PATH
            .set(Box::leak(path.to_owned().into_boxed_str()))
            .unwrap();
    });
    scorched::logf!(Info, "inside-configured-directory");
    assert!(logs.read().contains("[INFO] inside-configured-directory\n"));
}
