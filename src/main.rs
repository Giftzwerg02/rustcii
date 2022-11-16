use anyhow::Result;
use bit_vec::BitVec;
use clap::{arg, command, Command};

/// Bit-Value = 0
const ROCKET: char = '🚀';

/// Bit-Value = 1
const CRAB: char = '🦀';

fn main() {
    let matches = command!()
        .subcommand(Command::new("encode")
            .about("Converts your old and outdated \"characters\" into superior Rustcii-Encoding")
            .arg(arg!([input] "The input to encode")
                .required(true)
            )
        )
        .subcommand(Command::new("decode")
            .about("Converts superior Rustcii-Encoding into old and outdated \"characters\". Although I don't quite understand why you would want that...")
            .arg(arg!([input] "The input to decode")
                .required(true)
            )
        ).get_matches();

    matches.subcommand_matches("encode").map_or_else(
        || {
            if let Some(matches) = matches.subcommand_matches("decode") {
                if let Some(input) = matches.get_one::<String>("input") {
                    let decoded = decode(input);

                    match decoded {
                        Ok(decoded) => println!("{}", decoded),
                        Err(err) => println!("{}", err),
                    };
                }
            }
        },
        |matches| {
            if let Some(input) = matches.get_one::<String>("input") {
                let encoded = encode(input);
                println!("{}", encoded);
            }
        },
    );
}

fn encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let bits = bit_vec::BitVec::from_bytes(bytes);
    bits.iter()
        .map(|bit| if bit { CRAB } else { ROCKET })
        .collect()
}

fn decode(input: &str) -> Result<String> {
    let mut bits = BitVec::new();

    for c in input.chars() {
        let bit = match c {
            ROCKET => Ok(false),
            CRAB => Ok(true),
            _ => Err(anyhow::Error::msg(format!("Char was neither '{}' nor '{}'. I guess the input was created by some Go-Programmer...", ROCKET, CRAB)))
        }?;
        bits.push(bit);
    }

    String::from_utf8(bits.to_bytes()).map_err(anyhow::Error::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let original = "H";
        let actual = encode(original);
        // 01001000
        let expected =
            String::from_iter([ROCKET, CRAB, ROCKET, ROCKET, CRAB, ROCKET, ROCKET, ROCKET]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode() {
        let original =
            String::from_iter([ROCKET, CRAB, ROCKET, ROCKET, CRAB, ROCKET, ROCKET, ROCKET]);
        let actual = decode(&original).unwrap();
        let expected = "H";
        assert_eq!(actual, expected);
    }
}
