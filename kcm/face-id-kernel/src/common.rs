pub fn get_data_path() -> Option<std::path::PathBuf> {
    dirs::data_dir().map(|dir| dir.join("linux-hello"))
}
