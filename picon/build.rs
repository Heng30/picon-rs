use cmd_lib::run_fun;

fn main() {
    #[cfg(target_os = "windows")]
    {
        link_win_lib();
        set_win_info();
    }

    let _ = write_app_version();
}

fn write_app_version() -> Result<(), Box<dyn std::error::Error>> {
    let tags = run_fun!(git describe --tags --abbrev=0)?
        .split(char::is_whitespace)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    if let Some(version) = tags.last() {
        let output = format!(r#"pub const VERSION: &str = "{}";"#, version);
        let _ = std::fs::write("src/version.rs", output);
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn link_win_lib() {
    println!("cargo:rustc-link-search=win/lib");
}

#[cfg(target_os = "windows")]
fn set_win_info() {
    embed_resource::compile("../win/icon.rc", embed_resource::NONE);
}
