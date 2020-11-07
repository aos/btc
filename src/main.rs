use std::fs;
use std::io;
use std::env;
use std::str;
use std::collections::HashMap;

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
    let mut filename = "debian-10.6.0-amd64-netinst.iso.torrent";

    if args.len() > 1 {
        filename = &args[1];
    }

    let mut p = Parser::new(filename).unwrap();
    p.parse();

}

struct Parser {
    data: Vec<u8>,
    current_pos: usize,
    total_len: usize,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Parser, io::Error> {
        let data = fs::read(filename)?;
        let total_len = data.len();
        Ok(Self {
            data,
            current_pos: 0,
            total_len,
        })
    }

    pub fn parse(&mut self) -> Item {
        let x = Item::Byte(0);

        loop {
            let c = self.data[self.current_pos];

            match c as char {
                // d
                'd' => {
                    println!("we got a dict");
                    let n = self.get_number();
                    println!("n: {}", n);
                    println!("current_pos: {}", self.current_pos);
                    self.advance();
                    break
                }
                // l
                'l' => {
                    println!("list");
                    break
                }
                // i
                'i' => break,
                _ => break
            }
        }

        x
    }

    fn get_dict(&mut self, n: usize) {
        let data = self.data.get(self.current_pos..n);
    }

    fn get_number(&mut self) -> usize {
        let mut nums = Vec::new();

        while let Some(digit) = self.peek() {
            if digit.is_ascii_digit() {
                nums.push(digit);
                self.advance();
            } else {
                break
            }
        }
        let the_str = str::from_utf8(&nums[..]).unwrap();
        let the_num: usize = the_str.parse().unwrap();

        the_num
    }

    fn advance(&mut self) {
        self.current_pos += 1
    }

    fn peek(&self) -> Option<u8> {
        if !self.is_end() {
            Some(self.data[self.current_pos + 1])
        } else {
            None
        }
    }

    fn is_end(&self) -> bool {
        self.current_pos >= self.total_len
    }
}
