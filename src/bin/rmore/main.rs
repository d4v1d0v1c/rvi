// Usage: bmore [-acdir] [-lines] [+linenum | +/pattern] name1 name2 ...
use std::process;

mod app;
mod clap_app;
mod config;

use rmore::consts::COPYRIGHT;
use rmore::error::*;
use rmore::input::*;
use rmore::config::Config;

use crate::{
    app::App,
};


#[cfg(feature = "bugreport")]
fn invoke_bugreport(app: &App) {
    use bugreport::{bugreport, collector::*, format::Markdown};

    let mut report = bugreport!()
    .info(SoftwareVersion::default())
    .info(OperatingSystem::default())
    .info(CommandLine::default())
    .info(EnvironmentVariables::list(&[
            "RMORE_ASCII",
        ]))
    .info(CompileTimeInformation::default());
    report.print::<Markdown>();
}

fn run() -> Result<bool> {
    let app = App::new()?;
    
    if app.matches.get_flag("diagnostic") {
        #[cfg(feature = "bugreport")]
        invoke_bugreport(&app);
        #[cfg(not(feature = "bugreport"))]
        println!("rmore has been built without the 'bugreport' feature. The '--diagnostic' option is not available.");
        return Ok(true);
    }

    let inputs = app.inputs()?; // receive all files from cmd line.
    let config = app.config(&inputs)?;


    // BMore::default().run();
    Ok(true)
}

fn main() {
    println!("copyright: {COPYRIGHT:?}");
    let result = run();
    match result {
        Err(error) => {
            let stderr = std::io::stderr();
            default_error_handler(&error, &mut stderr.lock());
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}
