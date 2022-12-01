use clap::Parser;
use egg_smol::ast;
use egg_smol::egg_macro;
use egg_smol::EGraph;
use std::io::{self, BufRead};
use std::path::PathBuf;
#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'F', long)]
    fact_directory: Option<PathBuf>,
    #[clap(long)]
    naive: bool,
    inputs: Vec<PathBuf>,
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .parse_default_env()
        .init();

    let args = Args::parse();

    if args.inputs.is_empty() {
        let stdin = io::stdin();
        log::info!("Welcome to Egglog!");
        let mut egraph = EGraph::default();
        for line in stdin.lock().lines() {
            let line = line.unwrap_or_else(|_| panic!("Failed to read line from stdout"));
            match egraph.parse_and_run_program(&line) {
                Ok(_msgs) => {}
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }

        std::process::exit(1)
    }

    for (idx, input) in args.inputs.iter().enumerate() {
        let s = std::fs::read_to_string(input).unwrap_or_else(|_| {
            let arg = input.to_string_lossy();
            panic!("Failed to read file {arg}")
        });
        let parser = ast::parse::MacroProgramParser::new();
        let program = parser
            .parse(&s)
            .map_err(|e| e.map_token(|tok| tok.to_string()))
            .unwrap();
        let macros = program.into_iter().filter_map(|e| match e {
            ast::MacroCommand::MacroRewrite(r) => Some(r),
            _ => None,
        });
        let exprs = program
            .into_iter()
            .filter_map(|e| match e {
                ast::MacroCommand::Syntax(e) => {
                    Some(egg_macro::macro_expand(macros, e).to_string())
                }
                _ => None,
            })
            .collect();
        let s = exprs.join("\n");
        //let s = program.map(|e| e.to_string());

        let mut egraph = EGraph::default();
        egraph.fact_directory = args.fact_directory.clone();
        egraph.seminaive = !args.naive;
        match egraph.parse_and_run_program(&s) {
            Ok(_msgs) => {}
            Err(err) => {
                log::error!("{}", err);
                std::process::exit(1)
            }
        }

        // no need to drop the egraph if we are going to exit
        if idx == args.inputs.len() - 1 {
            std::mem::forget(egraph)
        }
    }
}
