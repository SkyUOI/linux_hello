use std::{ops::Not as _, process};

pub fn main() {
    let data_path = face_id_kernel::common::get_data_path()
        .unwrap()
        .join("database")
        .join("face-id-kernel");

    if data_path.exists().not() {
        std::fs::create_dir_all(data_path).unwrap();
    }

    std::env::set_current_dir("./kcm/face-id-kernel").unwrap();

    process::Command::new("sea-orm-cli")
        .arg("migrate")
        .args(std::env::args().skip(1))
        .output()
        .unwrap();
}
