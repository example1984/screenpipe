#[cfg(target_os = "windows")]
use std::{env, fs, io, process::Command};

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Set static CRT for Windows MSVC target
        if env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
            println!("cargo:rustc-env=KNF_STATIC_CRT=1");
            println!("cargo:rustc-flag=-C target-feature=+crt-static");
        }

        let url = "https://github.com/microsoft/onnxruntime/releases/download/v1.19.2/onnxruntime-win-x64-gpu-1.19.2.zip";
        let resp = reqwest::blocking::get(url).expect("request failed");
        let body = resp.text().expect("body invalid");
        let mut out =
            fs::File::create("onnxruntime-win-x64-gpu-1.19.2.zip").expect("failed to create file");
        io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
        let status = Command::new("unzip")
            .args(["onnxruntime-win-x64-gpu-1.19.2.zip"])
            .status()
            .expect("failed to execute process");
        if !status.success() {
            panic!("failed to install onnx binary");
        }
        fs::rename(
            "onnxruntime-win-x64-gpu-1.19.2",
            "../screenpipe-app-tauri/src-tauri/onnxruntime-win-x64-gpu-1.19.2",
        );
    }
}