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
    if num == 0 {
        println!("0");
        return;
    }
    let mut sum: u64 = 0;

    for n in 0..=num {
        let mut how_many_times_cepac: u8 = 0;
        println!("N {}", &sum);
        for base in 2..=36 {
            println!("How many times cepac {}", how_many_times_cepac);
            if base == 10 {
                println!("Base 10 break");
                continue;
            }
            if how_many_times_cepac == 2 {
                println!("How many times cepac 2");
                break;
            }

            let converted = convert_to_base(n, base);

            if converted.is_empty() {
                println!("Converted is empty");
                break;
            }

            if converted.chars().all(|ch| ch == '1') {
                match converted.parse::<u64>() {
                    Ok(num) => {
                        sum = sum + num;
                        println!("Sum {} added num {}", sum, num);
                        how_many_times_cepac += 1;
                    }
                    Err(_err) => {
                        break;
                    }
                }
            } else {
                continue;
            }
        }
    }
    println!("Sum {}", sum);
}

fn main() {
    let num = 10e3;
    convert_to_all_bases(num as u64);
}

#[cfg(test)]
mod test {
    use crate::convert_to_base;

    #[test]
    fn check_conversion() {
        let input = 511;
        let expected = String::from("111111111");

        assert_eq!(expected, convert_to_base(input, 2))
    }
}
