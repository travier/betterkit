// SPDX-License-Identifier: Apache-2.0 OR MIT

#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

use log::LevelFilter;
use structopt::StructOpt;
use zbus::{Connection, Result};


use zbus::interface;

#[derive(StructOpt, Debug)]
#[structopt(name = "betterkit", about = "Betterkit daemon")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    /// Enable testing under an unprivileged user with the user session bus
    #[structopt(
        short = "u",
        long = "user",
    )]
    user: bool,
}


struct BetterKit {
/*     motd_path: String */
}

#[interface(name = "org.betterkit.betterkit1")]
impl BetterKit {
    async fn Run(&self, argv: Vec<&str>) -> String {
        info!("Running: \"{}\"", argv.join("\" \""));
        use std::process::Command;
        let mut cmd = Command::new("systemd-run");
        cmd.arg("--user").arg("--pipe");
        for arg in argv {
            cmd.arg(arg);
        }
        let output = cmd.output()
                .expect("failed to execute process");
        String::from_utf8(output.stdout).expect("non utf8 output")
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let level = match opt.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::Builder::new()
        .filter(None, level)
        .format_timestamp(None)
        .init();

    let connection = if opt.user {
        Connection::session().await?
    } else {
        Connection::system().await?
    };

    connection.object_server().at("/org/betterkit/betterkit1", BetterKit {}).await?;

    connection.request_name("org.betterkit").await?;

    info!("Starting main loop");
    loop {
        std::future::pending::<()>().await;
    }
}

