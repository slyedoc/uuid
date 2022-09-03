use clap::{Parser, ArgEnum};
use uuid as ud; // renaming because so the cargo subcommand can be uuid

#[derive(Parser)] 
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    Uuid(Uuid),
}

/// Simple program to generate uuid's
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Uuid {
    // expected output formatted
    #[clap(short, long, arg_enum, default_value = "string")]
    output: Output,
}

#[derive(ArgEnum, Debug, PartialEq, Clone)]
#[clap(rename_all = "kebab_case")]
enum Output {
    String,
    U64,
    U128
}

fn main() {
    let Cargo::Uuid(args) = Cargo::parse();

    let id = ud::Uuid::new_v4();
    match args.output {
        Output::String => {
            println!("{}", id.to_string());
        },
        Output::U128 =>  {
            println!("{}", id.as_u128());
        }
        Output::U64 => {
            let pair = id.as_u64_pair();
            println!("{}", pair.0);
            println!("{}", pair.1);
        },
         
    }
    
    
}