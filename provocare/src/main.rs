use std::collections::HashMap;

fn convert_to_base(mut num: u64, base: u64) -> String {
    let mut result = String::new();

    while num > 0 {
        let digit = (num % base) as u8;
        result.insert(0, std::char::from_digit(digit as u32, base as u32).unwrap());
        num /= base;
    }

    result
}

#[derive(Debug)]
pub struct NumberCounting {
    pub how_many_times_cepac: u8,
    pub sum: u64,
}

fn convert_to_all_bases(num: u64) {
    let mut hashmap: HashMap<u64, NumberCounting> = HashMap::new();
    let mut sum: u64 = 0;
    for n in 0..=num {
        for base in 2..=36 {
            if base == 10 {
                break;
            }
            let converted = convert_to_base(n, base);

            if converted.chars().all(|ch| ch == '1') {
                match converted.parse::<u64>() {
                    Ok(num) => {
                        hashmap
                            .entry(n)
                            .and_modify(|entry| {
                                entry.how_many_times_cepac += 1;
                                sum += num;
                            })
                            .or_insert(NumberCounting {
                                how_many_times_cepac: 0,
                                sum: num,
                            });
                        println!("Base {} of number {} : {}", base, n, converted);
                    }
                    Err(err) => {
                        println!("Error parsing number: {:?}", err);
                        break;
                    }
                }
            }
        }
    }

    let _ = hashmap.into_iter().map(|entry| {
        if entry.1.how_many_times_cepac == 2 {
            sum = sum + entry.1.sum;
        }
    });

    println!("{:?}", sum); // Moved this line outside the inner loop
}

fn main() {
    let num = 1000; // Change this to the base 10 number you want to convert
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
