use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::iter::Map;

pub struct Day {
    hands: Vec<Hand>,
}

#[derive(Clone)]
pub struct Hand {
    hand: String,
    score: u8,
    bid: u64,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand == other.hand {
            true
        } else {
            false
        }
    }
}

impl Eq for Hand {}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand1: &Hand = &(self);
        let hand2: &Hand = &(other);

        if hand1.score > hand2.score {
            return Ordering::Greater;
        } else if hand1.score < hand2.score {
            return Ordering::Less;
        }


        let hand2bytes: Vec<char> = hand2.hand.chars().collect();
        for (index, card1) in hand1.hand.chars().collect::<Vec<char>>().iter().enumerate() {
            let card1_strength = get_card_strength(*card1);
            let card2_strength = get_card_strength(hand2bytes[index]);

            if card1_strength > card2_strength {
                return Ordering::Greater;
            } else if card1_strength < card2_strength {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut hands: Vec<Hand> = Default::default();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");

            let hand_bid = line.split(" ").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
            let bid = hand_bid[1].parse::<u64>().expect("should be a number");

            let score = get_score(hand_bid[0]);
            let hand = Hand {
                hand: hand_bid[0].to_string(),
                score: score,
                bid: bid,
            };

            hands.push(hand);
        }

        Self {
            hands,
        }
    }

    pub fn solve(&self) -> u64 {
        let mut sorted_hands = self.hands.clone();
        sorted_hands.sort();

        let mut total: u64 = Default::default();
        for (index, item) in sorted_hands.iter().enumerate() {
            println!("{} {} {} {}", item.hand, item.score, item.bid, index);

            total += item.bid * (index as u64 + 1);
        }

        total
    }
}

fn get_score(hand: &str) -> u8 {
    let mut strength: u8 = 0;
    let mut is_doubled = false;
    let mut hand_map: HashMap<u8, u8> = Default::default();
    for char in hand.as_bytes() {
        if hand_map.contains_key(char) {
            let new_value = hand_map[char] + 1;
            hand_map.insert(*char, new_value);
        } else {
            hand_map.insert(*char, 1);
        }

        if strength == 2 && hand_map[char] == 2 {
            is_doubled = true;
        }
        strength = cmp::max(strength, hand_map[char]);
    }

    if strength == 3 {
        if hand_map.len() == 2 {
            return 4;
        } else {
            return 3;
        }
    } else if strength == 2 {
        if is_doubled {
            return 2;
        } else {
            return 1;
        }
    } else if strength == 1 {
        return 0;
    }

    strength + 1
}

fn get_card_strength(char: char) -> u8 {
    if char == '2' {
        return 2;
    } else if char == '3' {
        return 3;
    } else if char == '4' {
        return 4;
    } else if char == '5' {
        return 5;
    } else if char == '6' {
        return 6;
    } else if char == '7' {
        return 7;
    } else if char == '8' {
        return 8;
    } else if char == '9' {
        return 9;
    } else if char == 'T' {
        return 10;
    } else if char == 'J' {
        return 11;
    } else if char == 'Q' {
        return 12;
    } else if char == 'K' {
        return 13;
    } else {
        return 14;
    }
}