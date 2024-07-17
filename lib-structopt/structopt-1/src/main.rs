use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Struct1 {
    #[structopt(flatten)]
    Struct1(Struct2),
}

#[derive(Debug, StructOpt)]
pub struct Struct2 {
    #[structopt(subcommand)]
    command_type: Enum3,
}

#[derive(Debug, StructOpt)]
pub enum Enum3 {
    Command { args: Vec<String> },
}

pub fn main() {
    let opt = Struct1::from_args();
    println!("{:?}", opt);
}