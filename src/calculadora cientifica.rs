use regex::Regex;

fn main() {
    println!("Hola platzi");
}

let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

println!("Por favor introduce tu expresion: ")
let mut expression = String::new();
std::io::stdin().read_line(&mut expression).unwrap();

loop {
    let caps = re_mult.captures(expression.as-string());

    if caps.is_none() {
        break;
    }

    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let mult = left_value * right_value

    expression = expression.replace(cap_expression, &addition.to_string());
}

loop{
    let caps = re_add.captures(expression.as-string());

    if caps.is_none() {
        break;
    }

    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value + right_value;

    expression = expression.replace(cap_expression, &addition.to_string());
}


println!("{:?} izq: {}, der: {}", caps, left_value, right_value);

println!("Resultado {}". expression);