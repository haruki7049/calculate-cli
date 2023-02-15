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
    let operator: &str = &args.operator;
    match operator {
        "addition" => args.first_parameter + args.second_parameter,
        "subtraction" => args.first_parameter - args.second_parameter,
        "multiplication" => args.first_parameter * args.second_parameter,
        "division" => args.first_parameter / args.second_parameter,
        _ => 0,//TODO: I want to return a ERROR parameter.
    }
}

#[test]
fn test_main(){
    let args: Args = Args::parse();
    let i = args.first_parameter + args.second_parameter;
    let j = args.first_parameter - args.second_parameter;
    let k = args.first_parameter * args.second_parameter;
    let l = args.first_parameter / args.second_parameter;

    println!("{}, {}, {}, {}", i, j, k, l);
}
