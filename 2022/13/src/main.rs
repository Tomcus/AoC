use nom::{
    IResult,
    character::{complete::char, complete::i64},
    combinator::opt,
    branch::alt,
    multi::many0,
    sequence::{terminated, delimited},
};
use std::io::BufRead;

#[derive(Debug, Clone, Eq, Ord)]
enum Item {
    Int(i64),
    List(Vec<Item>)
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Item::Int(is), &Item::Int(io)) => {
                is.partial_cmp(&io)
            },
            (&Item::List(ref vs), &Item::List(ref vo)) => {
                let mut it_s = vs.iter();
                let mut it_o = vo.iter();
                loop {
                    let left_val = it_s.next();
                    let right_val = it_o.next();

                    match (left_val, right_val) {
                        (Some(l_item), Some(r_item)) => {
                            let comp_res = l_item.partial_cmp(r_item).unwrap();
                            if comp_res == std::cmp::Ordering::Equal {
                                continue;
                            }
                            return Some(comp_res);
                        },
                        (None, None) => {
                            return Some(std::cmp::Ordering::Equal);
                        },
                        (Some(_), None) => {
                            return Some(std::cmp::Ordering::Greater);
                        },
                        (None, Some(_)) => {
                            return Some(std::cmp::Ordering::Less);
                        }
                    }
                }
            },
            (&Item::Int(a), &Item::List(ref b)) => {
                let tmp = Item::List(vec![Item::Int(a)]);
                tmp.partial_cmp(&Item::List(b.to_vec().clone()))
            },
            (a, &Item::Int(b)) => {
                a.partial_cmp(&Item::List(vec![Item::Int(b)]))
            }
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other).unwrap() == std::cmp::Ordering::Equal
    }
    
    fn ne(&self, other: &Self) -> bool {
        self.partial_cmp(&other).unwrap() != std::cmp::Ordering::Equal
    }
}

fn parse_item(input: &str) -> IResult<&str, Item> {
    let mut parser = alt((parse_list, parse_int));
    parser(input)
}

fn parse_list(input: &str) -> IResult<&str, Item> {
    let (rem, items) = delimited(char('['), many0(terminated(parse_item, opt(char(',')))), char(']'))(input)?;
    Ok((rem, Item::List(items)))
}

fn parse_int(input: &str) -> IResult<&str, Item> {
    let (rem, num) = i64(input)?;
    Ok((rem, Item::Int(num)))
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let divider_a = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let divider_b = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    let mut data = vec![
        divider_a.clone(),
        divider_b.clone()
    ];
    loop {
        let mut left_line = String::new();
        if reader.read_line(&mut left_line).unwrap() == 0 {
            break;
        }
        let mut right_line = String::new();
        assert!(reader.read_line(&mut right_line).unwrap() != 0);
        let mut empty_line = String::new();
        assert!(reader.read_line(&mut empty_line).unwrap() == 1);
        
        let left_packet = parse_item(&left_line).unwrap();
        let right_packet = parse_item(&right_line).unwrap();

        data.push(left_packet.1);
        data.push(right_packet.1);
    }

    let mut res = 1;
    data.sort_by(|a,b| a.partial_cmp(b).unwrap());
    for (i, item) in data.iter().enumerate() {
        if item.eq(&divider_a) || item.eq(&divider_b) {
            res *= i + 1;
            println!("{:3}: {:?}", i + 1, item);
        }
    }
    println!("{}", res);
}
