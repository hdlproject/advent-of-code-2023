use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

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

#[derive(Clone)]
pub struct Hand2 {
    hand: String,
    score: u8,
    bid: u64,
}

impl PartialEq<Self> for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        if self.hand == other.hand {
            true
        } else {
            false
        }
    }
}

impl Eq for Hand2 {}

impl PartialOrd<Self> for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand1: &Hand2 = &(self);
        let hand2: &Hand2 = &(other);

        if hand1.score > hand2.score {
            return Ordering::Greater;
        } else if hand1.score < hand2.score {
            return Ordering::Less;
        }

        let hand2bytes: Vec<char> = hand2.hand.chars().collect();
        for (index, card1) in hand1.hand.chars().collect::<Vec<char>>().iter().enumerate() {
            let card1_strength = get_card_strength2(*card1);
            let card2_strength = get_card_strength2(hand2bytes[index]);

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

            let hand_bid = line
                .split(" ")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>();
            let bid = hand_bid[1].parse::<u64>().expect("should be a number");

            let hand = Hand {
                hand: hand_bid[0].to_string(),
                score: 0,
                bid: bid,
            };

            hands.push(hand);
        }

        Self { hands }
    }

    pub fn solve(&self) -> u64 {
        let mut sorted_hands: Vec<Hand> = Default::default();
        for hand in &self.hands {
            sorted_hands.push(Hand {
                hand: hand.hand.clone(),
                score: get_score(hand.hand.as_str()),
                bid: hand.bid,
            });
        }
        sorted_hands.sort();

        let mut total: u64 = Default::default();
        for (index, item) in sorted_hands.iter().enumerate() {
            total += item.bid * (index as u64 + 1);
        }

        total
    }

    pub fn solve2(&self) -> u64 {
        let mut sorted_hands: Vec<Hand2> = Default::default();
        for hand in &self.hands {
            sorted_hands.push(Hand2 {
                hand: hand.hand.clone(),
                score: get_score2(hand.hand.as_str()),
                bid: hand.bid,
            });
        }
        sorted_hands.sort();

        let mut total: u64 = Default::default();
        for (index, item) in sorted_hands.iter().enumerate() {
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
            // full house
            return 4;
        } else {
            // three of a kind
            return 3;
        }
    } else if strength == 2 {
        if is_doubled {
            // two pair
            return 2;
        } else {
            // one pair
            return 1;
        }
    } else if strength == 1 {
        // high card
        return 0;
    }

    // (5) four of a kind / (6) five of a kind
    strength + 1
}

fn get_score2(hand: &str) -> u8 {
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

    let mut jocker_count: u8 = Default::default();
    for char in hand.chars() {
        if char == 'J' {
            jocker_count = jocker_count + 1;
        }
    }

    if strength == 4 {
        if jocker_count > 0 {
            // (6) five of a kind
            return 6;
        }
        // (5) four of a kind
        return 5;
    } else if strength == 3 {
        if hand_map.len() == 2 {
            if jocker_count > 0 {
                // (6) five of a kind
                return 6;
            }
            // full house
            return 4;
        } else {
            if jocker_count > 0 {
                // (5) four of a kind
                return 5;
            }
            // three of a kind
            return 3;
        }
    } else if strength == 2 {
        if is_doubled {
            if jocker_count > 0 {
                // full house / (5) four of a kind
                return 2 + 1 + jocker_count;
            }
            // two pair
            return 2;
        } else {
            if jocker_count > 0 {
                // three of a kind
                return 3;
            }
            // one pair
            return 1;
        }
    } else if strength == 1 {
        if jocker_count > 0 {
            // one pair
            return 1;
        }
        // high card
        return 0;
    }

    // (6) five of a kind
    6
}

fn get_card_strength(char: char) -> u8 {
    if char == '2' {
        2
    } else if char == '3' {
        3
    } else if char == '4' {
        4
    } else if char == '5' {
        5
    } else if char == '6' {
        6
    } else if char == '7' {
        7
    } else if char == '8' {
        8
    } else if char == '9' {
        9
    } else if char == 'T' {
        10
    } else if char == 'J' {
        11
    } else if char == 'Q' {
        12
    } else if char == 'K' {
        13
    } else {
        14
    }
}

fn get_card_strength2(char: char) -> u8 {
    if char == '2' {
        2
    } else if char == '3' {
        3
    } else if char == '4' {
        4
    } else if char == '5' {
        5
    } else if char == '6' {
        6
    } else if char == '7' {
        7
    } else if char == '8' {
        8
    } else if char == '9' {
        9
    } else if char == 'T' {
        10
    } else if char == 'J' {
        1
    } else if char == 'Q' {
        12
    } else if char == 'K' {
        13
    } else {
        14
    }
}
