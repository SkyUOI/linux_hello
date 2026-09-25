fn kf6_include_dirs() -> Vec<String> {
    cmake_package::find_package("KF6KCMUtils")
        .find()
        .expect("Couldn't find KF6KCMUtils, please install it")
        .target("KF6::KCMUtilsQuick")
        .expect("KF6::KCMUtilsQuick doesn't exist in KF6KCMUtils")
        .include_directories
}

fn main() {}
