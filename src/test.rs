#[test]
fn formats_line_deterministically() {
    assert_eq!(
        crate::time_utils::format_log_line("ERROR", "boom", "2026-09-14 16:07:03"),
        "2026-09-14 16:07:03 [ERROR] boom\n"
    );
}
