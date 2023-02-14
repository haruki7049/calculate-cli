use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    operator: String,

    #[arg(short, long)]
    first_parameter: i32,

    #[arg(short, long)]
    second_parameter: i32,
}

fn main(){
    let args: Args = Args::parse();
    let i: i32 = calculate_parameter(&args);

    println!("{}", i);
}

fn calculate_parameter(args: &Args) -> i32{
    let operator: String = args.operator;
    match operator {
        "+" => args.first_parameter + args.second_parameter
        _ => 0
    }
}
