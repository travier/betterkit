// SPDX-License-Identifier: Apache-2.0 OR MIT

#![forbid(unsafe_code)]

#[macro_use]
extern crate log;
extern crate serde;

use log::LevelFilter;
use structopt::StructOpt;
use zbus::{Connection, Result};
use zbus::interface;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: u64,
    argv: Vec<String>,
    status: String,
    stdout: String,
    stderr: String,
}

impl Transaction {
    fn new(id: u64, argv: Vec<String>) -> Transaction {
        Transaction {
            id,
            argv,
            status: String::from("New"),
            stdout: String::new(),
            stderr: String::new(),
        }
    }
}

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
    transactions: HashMap<u64, Transaction>,
    next_transaction_id: u64,
}

#[interface(name = "org.betterkit.betterkit1")]
impl BetterKit {
    async fn Run(&mut self, argv: Vec<&str>) -> String {
        info!("Creating transaction: {}", self.next_transaction_id);
        let argv_string = argv.into_iter().map(|x| String::from(x)).collect();
        let transaction = Transaction::new (self.next_transaction_id, argv_string);
        self.transactions.insert(self.next_transaction_id, transaction);
        self.next_transaction_id += 1;
        format!("{{ \"id\": \"{}\" }}", self.next_transaction_id - 1)
    }

    async fn Get(&mut self, id: u64) -> String {
        info!("Getting transaction: {}", id);
        match self.transactions.get(&id) {
            Some(t) => format!("{{ \"id\": \"{}\" }}", t.id),
            None => format!("No transaction found for: {}", id),
        }        
    }
}

/*
fn run() {
        info!("Running: \"{}\"", argv.join("\" \""));
        use std::process::Command;
        let mut cmd = Command::new("systemd-run");
        cmd.arg("--user").arg("--pipe");
        for arg in argv {
            cmd.arg(arg);
        }
        let output = cmd.output()
                .expect("failed to execute process");
        String::from_utf8(output.stdout).expect("non utf8 output")}
*/

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

    let context = BetterKit {
        transactions: HashMap::new(),
        next_transaction_id: 0,
    };
    connection.object_server().at("/org/betterkit/betterkit1", context).await?;

    connection.request_name("org.betterkit").await?;

    info!("Starting main loop");
    loop {
        std::future::pending::<()>().await;
    }
}

