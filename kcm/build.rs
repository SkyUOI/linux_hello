use std::{
    fs,
    io::Write,
    path::{self, PathBuf},
};

use anyhow::Context;
use cxx_qt_build::CppFile;

fn main() -> anyhow::Result<()> {
    let include_dirs = kcm_include_dirs()?;

    let mut builder =
        cxx_qt_build::CxxQtBuilder::new_qml_module(cxx_qt_build::QmlModule::new("org.kde.libkcm"))
            .file("src/lib.rs")
            .cpp_file(CppFile::from("src/kcm_plugin.h").moc_arguments({
                let mut args = cxx_qt_build::MocArguments::default();
                args = args.include_paths(include_dirs.iter());
                args
            }))
            .qrc("src/qml.qrc");

    unsafe {
        builder = builder.cc_builder(|cc| {
            cc.flag_if_supported("-Wno-sfinae-incomplete");
            include_dirs.iter().for_each(|dir| {
                let _ = cc.include(dir);
            });
        })
    }

    let version_script = get_version_script();

    let exports_path = get_exports_path()?;

    let mut exports_file = get_exports_file(&exports_path)?;

    for symbol in KCM_EXPORT_SYMBOLS {
        println!("cargo::rustc-link-arg=-Wl,--undefined={symbol}")
    }

    exports_file.write_all(version_script.as_bytes())?;

    println!(
        "cargo::rustc-link-arg=-Wl,--version-script={}",
        exports_path.display()
    );

    builder.build();

    Ok(())
}

fn kf6_include_dirs() -> anyhow::Result<Vec<String>> {
    cmake_package::find_package("KF6KCMUtils")
        .find()
        .context("Couldn't find KF6KCMUtils, please install it")?
        .target("KF6::KCMUtilsQuick")
        .map(|target| {
            target.link();
            target.include_directories
        })
        .context("KF6::KCMUtilsQuick doesn't exist in KF6KCMUtils")
}

fn qt6_multimedia_include_dirs() -> anyhow::Result<Vec<String>> {
    cmake_package::find_package("Qt6Multimedia")
        .find()
        .context("Couldn't find Qt6Multimedia, please install it")?
        .target("Qt6::Multimedia")
        .map(|target| {
            target.link();
            target.include_directories
        })
        .context("Qt6::Multimedia doesn't exist in Qt6Multimedia")
}

fn qt6_concurrent_include_dirs() -> anyhow::Result<Vec<String>> {
    cmake_package::find_package("Qt6")
        .components(&["Concurrent".to_owned()])
        .find()
        .context("Couldn't find Qt6 with Concurrent, please install it")?
        .target("Qt6::Concurrent")
        .map(|target| {
            target.link();
            target.include_directories
        })
        .context("Qt6::Concurrent doesn't exist in Qt6's Concurrent component")
}

fn kcm_include_dirs() -> anyhow::Result<Vec<String>> {
    let mut kcm_dirs = kf6_include_dirs()?;
    let multimedia_dirs = qt6_multimedia_include_dirs()?;
    let concurrent_dirs = qt6_concurrent_include_dirs()?;
    kcm_dirs.extend_from_slice(&multimedia_dirs);
    kcm_dirs.extend_from_slice(&concurrent_dirs);
    Ok(kcm_dirs)
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

fn get_exports_path() -> anyhow::Result<PathBuf> {
    Ok(
        path::Path::new(&std::env::var("OUT_DIR").context("env 'OUT_DIR' not found")?)
            .join("qt-plugin-exports.txt"),
    )
}

fn get_exports_file(exports_path: &path::Path) -> anyhow::Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(exports_path)
        .context("cannot open and write qt-plugin-exports.txt")
}
