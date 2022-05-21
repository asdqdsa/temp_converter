fn main() {
    println!("Waiting INPUT<num>");
    let num_temp :f32 = user_input().parse::<f32>().expect("WRONG_TYPE");
    println!("Waiting INPUT<char>");
    let char_temp :char = user_input().parse::<char>().expect("WRONG_TYPE");
    println!("{:.2}{}", convert(num_temp, char_temp), char_temp);
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error, can't read the input");
    input.trim().to_string()
}

fn convert(num: f32, type_temp: char) -> f32 {
    match type_temp {
        'f' | 'F' => convert_far_to_cel(num),
        'c' | 'C' => convert_cel_to_far(num),
        _ => panic!("no such type"),
    }
}

// converting Far -> Cels
fn convert_far_to_cel(tempr: f32) -> f32 {
    (tempr - 32.0) * 5.0 / 9.0
}
// converting Cels -> Far
fn convert_cel_to_far(tempr: f32) -> f32 {
    tempr / 5.0 * 9.0 + 32.0
}
