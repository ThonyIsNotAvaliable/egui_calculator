pub fn calculate_operations(first_input: f64, second_input: f64, operation: i8) -> f64 {
    if operation == 0 {      // Plus
        first_input + second_input
    }
    else if operation == 1 { // Minus
        first_input - second_input
    }
    else if operation == 2 { // Multiply
        first_input * second_input
    }
    else if operation == 3 { // Divide
        first_input / second_input
    }
    else {
        0.0
    }
}

pub fn sqrt(first_input: f64) -> f64 {
    first_input.abs().sqrt()
}


pub fn get_root(number: f64, power: f64, increment: f64, start: f64) -> f64 {

    let mut internal = start;
    let mut last: f64 = 0.0;
    let mut result: f64 = 0.0;

    if number != 0.0 && power != 0.0 && increment != 0.0 {

    

    while result < number.abs() {
        last = internal;
        result =  internal.powf(power);
        internal += increment.abs();
    }
}
    last
}

pub fn parse_text(text: &String) -> f64 {
    match text.parse::<f64>() {
        Ok(result) => result,
        Err(_) => 0.0
    }
}
