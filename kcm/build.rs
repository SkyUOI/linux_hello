use std::{
    fs,
    io::Write,
    path::{self, PathBuf},
    process::{self, ExitCode},
};

use cxx_qt_build::CppFile;

fn main() -> process::ExitCode {
    let include_dirs = match kf6_include_dirs() {
        Ok(include_dirs) => include_dirs,
        Err(e) => {
            println!("cargo::error=build.rs failed: {e}");
            return process::ExitCode::FAILURE;
        }
    };

    let mut builder = cxx_qt_build::CxxQtBuilder::new()
        .file("src/lib.rs")
        .cpp_file(CppFile::from("src/kcm_plugin.h").moc_arguments({
            let mut args = cxx_qt_build::MocArguments::default();
            args = args.include_paths(include_dirs.iter());
            args
        }))
        .qrc("src/qml.qrc");

    unsafe {
        builder = builder.cc_builder(|cc| {
            include_dirs.iter().for_each(|dir| {
                let _ = cc.include(dir);
            });
        })
    }

    let version_script = get_version_script();

    let exports_path = match get_exports_path() {
        Ok(export_path) => export_path,
        Err(e) => {
            println!("cargo::error=build.rs failed: {e}");
            return process::ExitCode::FAILURE;
        }
    };

    let mut exports_file = match get_exports_file(&exports_path) {
        Ok(exports_file) => exports_file,
        Err(e) => {
            println!("cargo::error=build.rs failed: {e}");
            return ExitCode::FAILURE;
        }
    };

    for symbol in KCM_EXPORT_SYMBOLS {
        println!("cargo::rustc-link-arg=-Wl,--undefined={symbol}")
    }

    if let Err(e) = exports_file.write(version_script.as_bytes()) {
        println!("cargo::error=build.rs failed: {e}");
        return ExitCode::FAILURE;
    }

    println!(
        "cargo::rustc-link-arg=-Wl,--version-script={}",
        exports_path.display()
    );

    builder.build();

    process::ExitCode::SUCCESS
}

fn kf6_include_dirs() -> Result<Vec<String>, &'static str> {
    cmake_package::find_package("KF6KCMUtils")
        .find()
        .map_err(|_| "Couldn't find KF6KCMUtils, please install it")
        .and_then(|package| {
            package
                .target("KF6::KCMUtilsQuick")
                .ok_or("KF6::KCMUtilsQuick doesn't exist in KF6KCMUtils")
        })
        .map(|target| target.include_directories)
}

const KCM_EXPORT_SYMBOLS: &[&str] = &["qt_plugin_instance", "qt_plugin_query_metadata_v2"];

fn get_version_script() -> String {
    let mut version_script = r"{
    global:
"
    .to_owned();
    for symbol in KCM_EXPORT_SYMBOLS {
        version_script.push_str(&format!("\t\t{symbol};\n"));
    }
    version_script.push_str("};");
    version_script
}

fn get_exports_path() -> Result<PathBuf, &'static str> {
    Ok(
        path::Path::new(&std::env::var("OUT_DIR").map_err(|_| "env 'OUT_DIR' not found")?)
            .join("qt-plugin-exports.txt"),
    )
}

fn get_exports_file(exports_path: &path::Path) -> Result<fs::File, &'static str> {
    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(exports_path)
        .map_err(|_| "cannot open and write qt-plugin-exports.txt")
}
