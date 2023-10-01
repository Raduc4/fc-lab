fn convert_to_base(mut num: u64, base: u64) -> String {
    let mut result = String::new();

    while num > 0 {
        let digit = (num % base) as u8;
        result.insert(0, std::char::from_digit(digit as u32, base as u32).unwrap());
        num /= base;
    }

    result
}

fn convert_to_all_bases(num: u64) {
    let mut sum: u64 = 0;
    for n in 0..=num {
        let mut should_break = false;
        for base in 2..=36 {
            let converted = convert_to_base(n, base);

            for ch in converted.chars() {
                if ch != '1' {
                    should_break = true;
                    break;
                }
            }

            if should_break {
                break;
            } else {
                match converted.parse::<u64>() {
                    Ok(num) => {
                        sum = sum + num;
                    }
                    Err(err) => {
                        break;
                    }
                }
            }
            println!("Sum {}", sum);
        }
    }
}

fn main() {
    let num = 10e6; // Change this to the base 10 number you want to convert
    convert_to_all_bases(num as u64);
}
