use std::fs;
use std::env;
use std::str;
use std::collections::HashMap;
use std::iter::Peekable;

// This is a copy of serde's Value
enum Item {
    Byte(u8),
    Str(String),
    Int(u64),
    List(Vec<Item>),
    Dict(HashMap<String, Item>)
}

// bencoding
// strings are length-prefixed base ten followed by a colon and string
// integers: i<number>e
// lists: l<element>e
// dictionary: d<element>e
// ex: d3:cow3:moo4:spam4:eggse -> {'cow': 'moo', 'spam': 'eggs'}
// ex: d3:cow3:mood4:phin4:cuteee -> {'cow': 'moo', {'phin': 'cute'}}
// ex: d4:spaml1:a1:bee -> {'spam': ['a', 'b']}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let data = fs::read(filename).expect("unable to read file");
    let mut it = data.iter().peekable();

    while let Some(&c) = it.peek() {
        match *c {
            b'd' => {
                // do something with the dictionary
                it.next(); // d
                let n = get_number(&mut it);
                let c_it = it.clone().take(n);
                println!("n = {}", n);
                break
            },
            b'l' => {
                // do something with the list
            },
            _ => continue
        }
    }
}

fn get_number<'a, I>(iter: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = &'a u8>
{
    let mut nums = Vec::new();

    while let Some(&digit) = iter.peek() {
        if digit.is_ascii_digit() {
            nums.push(*digit);
            iter.next();
        } else {
            break
        }
    }
    let the_str = str::from_utf8(&nums[..]).unwrap();
    let the_num: usize = the_str.parse().unwrap();

    the_num
}
