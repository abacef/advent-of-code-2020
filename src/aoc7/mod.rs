use std::{fs, fmt};
use regex::Regex;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc7/aoc7.txt";
const ONLY_BAGS_REGEX: &str = r"( bags contain no other bags.)|( bags contain \d )|( bags, \d )|( bag, \d )|( bags.)|( bag.)";
const BAG_WITH_MULT_REGEX: &str = r"( bags contain no other bags.)|( bags contain )|( bags, )|( bag, )|( bags.)|( bag.)";

struct BagContents {
    bag_name: u64,
    contents: Vec<u64>
}

impl Display for BagContents {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.bag_name, self.contents)
    }
}

fn hashed_target_bag() -> u64 {
    let mut hasher = DefaultHasher::new();
    "shiny gold".hash(&mut hasher);
    hasher.finish()
}

pub fn solve1() -> u32 {
    let mut reversed_graph: HashMap<u64, Vec<u64>> = HashMap::new();
    let only_bags_regex: Regex = Regex::new(ONLY_BAGS_REGEX).unwrap();

    // create graph
    let given_graph: Vec<BagContents> = fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| {
            print!("line: {}\nvalues: ", x);
            let split_line = only_bags_regex.split(x);
            let val = split_line.filter(|y| y.len() != 0).map(|y| {
                let mut hasher = DefaultHasher::new();
                y.hash(&mut hasher);
                let ret = hasher.finish();
                println!("{}, {}", y, ret);
                ret
            });
            println!();
            val
        })
        .map(|mut x| BagContents {
            bag_name: x.next().unwrap(),
            contents: x.collect()
        })
        .collect();

    // reverse graph
    for x in given_graph {
        println!("{}", x);
        for y in x.contents.iter() {
            let entry = reversed_graph.get_mut(y);
            if entry.is_some() {
                entry.unwrap().push(x.bag_name)
            } else {
                let new_vec = vec!(x.bag_name);
                reversed_graph.insert(*y, new_vec);
            }
        }
    };

    let mut tran_closure: HashSet<u64> = HashSet::new();
    let target_bag = hashed_target_bag();
    let mut neighbors = reversed_graph.get(&target_bag).unwrap().clone();
    for x in &neighbors {
        tran_closure.insert(*x);
    }

    // compute trans closure
    loop {
        let mut new_neighbors: Vec<u64> = vec!();
        for x in &neighbors {
            println!("{}", x);
            let next_container = reversed_graph.get(x);
            match next_container {
                Some(y) => new_neighbors.extend_from_slice(y),
                None => continue
            }
        }
        if new_neighbors.len() == 0 {
            return tran_closure.len() as u32;
        } else {
            for x in &new_neighbors {
                tran_closure.insert(*x);
            }
            neighbors = new_neighbors;
        }
    }
}

struct Bag {
    bag: String,
    mult: u8
}

struct BagWithMult {
    bag: String,
    contents: Vec<Bag>
}

fn recurse(map: &HashMap<String, Vec<Bag>>, to_find: &str) -> u64 {

    let arr = map.get(to_find).unwrap();
    match arr.len() {
        0 => 0,
        _ => {
            let mut acc: u64 = 0;
            for x in arr {
                let recurse_result = recurse(map, &x.bag);
                let tot_result = x.mult as u64 + x.mult as u64 * recurse_result;
                acc += tot_result;
                println!("{} + {} * {} = {}", x.mult, x.mult, recurse_result, tot_result);
            }
            print!("in: {}!", to_find);
            println!();
            acc
        }
    }
}

pub fn solve2() -> u64 {
    let split_regex = Regex::new(BAG_WITH_MULT_REGEX).unwrap();
    let mut map = HashMap::new();

    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| {
            let mut split_line = split_regex.split(x)
                .filter(|y| y.len() != 0);
            let enclosing = split_line.next().unwrap();
            let mut contents = vec!();
            loop {
                match split_line.next() {
                    Some(s) => {
                        let bytes = s.as_bytes();
                        let mult = bytes[0] - 48;
                        println!("{}", mult);
                        let mut iter = bytes.iter();
                        iter.next();
                        iter.next();
                        contents.push(Bag {
                            bag: String::from_utf8(iter.map(|y| *y).collect::<Vec<u8>>()).unwrap(),
                            mult
                        });
                    },
                    None => break
                }
            }
            BagWithMult {
                bag: enclosing.to_string(),
                contents
            }
        })
        .for_each(|x| {
            map.insert(x.bag, x.contents);
            ()
        });

    let answer = recurse(&map, "shiny gold");

    answer
}