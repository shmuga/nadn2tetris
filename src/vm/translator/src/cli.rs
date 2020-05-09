use clap::{load_yaml, App};

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output: String,
}

pub fn args() -> Args {
    let yml = load_yaml!("cli.yaml");
    let matches = App::from(yml).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("output").unwrap();

    Args {
        input: String::from(input),
        output: String::from(output)
    }
}
