use std::{env, fs, path::PathBuf};
use crate::util::render_template;

pub fn gen_man() -> anyhow::Result<()> {
    println!("cargo:rerun-if-env-changed=PROJECT_NAME");
    println!("cargo:rerun-if-env-changed=PROJECT_EXECUTABLE");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=RMORE_ASSETS_GEN_DIR");

    let project_name = env::var("PROJECT_NAME").unwrap_or("rmore".into());
    let executable_name = env::var("PROJECT_EXECUTABLE").unwrap_or(project_name.clone());
    let executable_name_uppercase = executable_name.to_uppercase();
    let project_version = env::var("CARGO_PKG_VERSION")?;

    let variables = [
        ("PROJECT_NAME", project_name),
        ("PROJECT_EXECUTABLE", executable_name),
        ("PROJECT_EXECUTABLE_UPPERCASE", executable_name_uppercase),
        ("PROJECT_VERSION", project_version),
    ]
    .into_iter()
    .collect();
    let Some(out_dir) = env::var_os("RMORE_ASSETS_GEN_DIR")
        .or_else(|| env::var_os("OUT_DIR"))
        .map(PathBuf::from)
    else {
        anyhow::bail!("RMORE_ASSETS_GEN_DIR or OUT_DIR should be set for build.rs");
    };

    fs::create_dir_all(out_dir.join("assets/manual")).unwrap();
    render_template(
        &variables,
        "assets/manual/rmore.1.in",
        out_dir.join("assets/manual/rmore.1"),
    )?;
/*

    render_template(
        &variables,
        "assets/completions/rmore.bash.in",
        out_dir.join("assets/completions/rmore.bash"),
    )?;
    render_template(
        &variables,
        "assets/completions/rmore.fish.in",
        out_dir.join("assets/completions/rmore.fish"),
    )?;
    render_template(
        &variables,
        "assets/completions/_rmore.ps1.in",
        out_dir.join("assets/completions/_rmore.ps1"),
    )?;
    render_template(
        &variables,
        "assets/completions/rmore.zsh.in",
        out_dir.join("assets/completions/rmore.zsh"),
    )?;

    println!(
        "cargo:rustc-env=RMORE_GENERATED_COMPLETION_BASH={}",
        out_dir.join("assets/completions/rmore.bash").display()
    );
    println!(
        "cargo:rustc-env=RMORE_GENERATED_COMPLETION_FISH={}",
        out_dir.join("assets/completions/rmore.fish").display()
    );
    println!(
        "cargo:rustc-env=RMORE_GENERATED_COMPLETION_PS1={}",
        out_dir.join("assets/completions/_rmore.ps1").display()
    );
    println!(
        "cargo:rustc-env=RMORE_GENERATED_COMPLETION_ZSH={}",
        out_dir.join("assets/completions/rmore.zsh").display()
    );
 */
    Ok(())
}