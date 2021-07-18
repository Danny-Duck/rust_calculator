use std::env::args;

// 4 actions are accepted,
// ADD, SUBTRACT, MULTIPLY and DIVIDE
enum Action {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

fn get_action_enum(s: &str) -> Action {
    use Action::*;
    match s {
        "add" => ADD,
        "subtract" => SUBTRACT,
        "multiply" => MULTIPLY,
        "divide" => DIVIDE,
        _ => panic!("Action \"{}\" not supported, try again", s),
    }
}

fn get_operands(vec: &[String]) -> Vec<isize> {
    let mut res = Vec::new();
    for str in vec {
        res.push(match str.parse::<isize>() {
            Ok(num) => num,
            Err(_) => panic!("\"{}\" is not an integer, try again", str),
        })
    }
    res
}

fn execute(action: Action, operands: Vec<isize>) -> String {
    use Action::*;
    let sum;
    match action {
        ADD => sum = operands[0] + operands[1],
        SUBTRACT => sum = operands[0] - operands[1],
        MULTIPLY => sum = operands[0] * operands[1],
        DIVIDE => sum = operands[0] / operands[1],
    }
    sum.to_string()
}

fn main() {
    // take 3 arguments, action and 2 operands
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        println!("expect 3 arguments got {}", args.len());
        println!("{:?}", args);
        return;
    }

    let operands = get_operands(&args[(2..=3)]);
    let action = get_action_enum(&args[1]);

    println!("{}", execute(action, operands))
    // calculator <action> <operands (max 2)>
    // > calculator add 3 4 = 7
    // > calculator subtract 30 15 = 15
    // > calculator multiply 30 15 = 450
    // > calculator divide 30 15 (only returns whole numbers)
}
