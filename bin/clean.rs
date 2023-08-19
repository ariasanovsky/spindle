fn main() {
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let spindle_dir = format!("{target_dir}/spindle");
    std::fs::remove_dir_all(spindle_dir).unwrap();
}
