fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    println!("args: {args:?}");

    let first_num: f64;
    let second_num: f64;

    let first_arg = args.get(0);
    println!("first_arg: {first_arg:?}");
    if let Some(first_string) = first_arg {
        let first_num_result = first_string.parse::<f64>();
        println!("first_num_result: {first_num_result:?}");
        if let Ok(first_value) = first_num_result {
            println!("first_num: {first_value:?}");
            first_num = first_value;
        } else {
            eprintln!("first argument is not a number");
            return;
        }
    } else {
        eprintln!("did not receive first argument");
        return;
    }

    let third_arg = args.get(2);
    println!("third_arg: {third_arg:?}");
    if let Some(third_string) = third_arg {
        let third_num_result = third_string.parse::<f64>();
        println!("third_num_result: {third_num_result:?}");
        if let Ok(third_value) = third_num_result {
            println!("third_num: {third_value:?}");
            second_num = third_value;
        } else {
            return;
        }
    } else {
        return;
    }

    let second_arg = args.get(1);
    println!("second_arg: {second_arg:?}");
    if let Some(second_string) = second_arg {
        let result = match second_string.as_str() {
            "/" => {
                first_num / second_num
            }
            "*" => {
                first_num * second_num
            }
            "+" => {
                first_num + second_num
            }
            "-" => {
                first_num - second_num
            }
            other => {
                eprintln!("{other} is not a valid operator");
                return;
            }
        };
        println!("result = {result}");
    }
}
