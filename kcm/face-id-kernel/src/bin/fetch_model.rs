use std::ops::Not as _;

use anyhow::Context as _;
use fern::colors;

fn log_init() -> anyhow::Result<()> {
    let colors: colors::ColoredLevelConfig = colors::ColoredLevelConfig::new()
        .info(colors::Color::Green)
        .debug(colors::Color::Blue);
    Ok(fern::Dispatch::new()
        .format(
            move |call_back: fern::FormatCallback<'_>, argument, record| {
                let when = jiff::Zoned::now();
                call_back.finish(format_args!(
                    "\x1B[{}m[{} {} {}:{}] {}\x1B[0m",
                    colors.get_color(&record.level()).to_fg_str(),
                    when.strftime("%Y-%m-%d %H:%M:%S%.3f %:z"),
                    record.level(),
                    record.file().unwrap_or("<unknown>"),
                    record.line().unwrap_or(0),
                    argument
                ))
            },
        )
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_init()?;

    let client = hf_hub::HFClient::new()?;

    let model_path = face_id_kernel::common::get_data_path()
        .context("cannot get config path")?
        .join("models")
        .join("face-id-kernel");

    if model_path.exists().not() {
        std::fs::create_dir_all(&model_path)?;
    }
    if model_path.join("2.5g_bnkps.onnx").exists().not() {
        let scrfd_path = client
            .model("RuteNL", "SCRFD-face-detection-ONNX")
            .download_file()
            .filename("2.5g_bnkps.onnx")
            .local_dir(&model_path)
            .send()
            .await?;
        log::info!(
            "downloaded SCRFD model to {path}",
            path = scrfd_path.display()
        );
    }

    if model_path.join("w600k_mbf.onnx").exists().not() {
        let buffalo_s_realpath = model_path.join("w600k_mbf.onnx");
        let buffalo_s_path = client
            .model("deepghs", "insightface")
            .download_file()
            .filename("buffalo_s/w600k_mbf.onnx")
            .local_dir(&model_path)
            .send()
            .await?;

        std::fs::rename(&buffalo_s_path, &buffalo_s_realpath)?;
        std::fs::remove_dir(
            buffalo_s_path
                .parent()
                .context("buffalo_s_path has no parent")?,
        )?;

        log::info!(
            "downloaded w600k_mbf model to {path}",
            path = buffalo_s_realpath.display()
        );
    }

    if model_path.join("genderage.onnx.onnx").exists().not() {
        let buffalo_s_realpath = model_path.join("genderage.onnx");
        let buffalo_s_path = client
            .model("deepghs", "insightface")
            .download_file()
            .filename("buffalo_s/genderage.onnx")
            .local_dir(&model_path)
            .send()
            .await?;

        std::fs::rename(&buffalo_s_path, &buffalo_s_realpath)?;
        std::fs::remove_dir(
            buffalo_s_path
                .parent()
                .context("buffalo_s_path has no parent")?,
        )?;

        log::info!(
            "downloaded genderage.onnx model to {path}",
            path = buffalo_s_realpath.display()
        );
    }

    Ok(())
}
