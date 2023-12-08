use std::fs;
use std::cmp::Ordering;
use regex::Regex;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Jack = 1
}

fn string_to_card(string: &str) -> Card {
    match string {
        "A" => { Card::Ace },
        "K" => { Card::King },
        "Q" => { Card::Queen },
        "J" => { Card::Jack },
        "T" => { Card::Ten },
        "9" => { Card::Nine },
        "8" => { Card::Eight },
        "7" => { Card::Seven },
        "6" => { Card::Six },
        "5" => { Card::Five },
        "4" => { Card::Four },
        "3" => { Card::Three },
        "2" => { Card::Two },
        _ => { panic!("Unkown card!") }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i64
}

impl Hand {
    fn build_hand(line: &str) -> Self {
        let re = Regex::new(r"(?<hand>\w{5}) (?<bid>\d+)").unwrap();
        let cap = re.captures(line).unwrap();

        let mut cards : Vec<Card> = vec![];
        for char in cap["hand"].chars() {
            cards.push(string_to_card(&char.to_string()));
        }
        let bid = cap["bid"].parse::<i64>().unwrap();
        Hand {
            cards,
            bid
        }
    }

    fn hand_type_from_vec(vec: Vec<Card>) -> HandType {
        let mut deduped = vec.to_vec();
        deduped.sort();
        deduped.dedup();
        let len = deduped.len();
        if len == 1 {
            HandType::FiveOfAKind
        } else if len == 5 {
            HandType::HighCard
        } else if len == 4 {
            HandType::OnePair
        } else if len == 2 {
            let count = vec.iter().filter(|&c| *c == *deduped.first().unwrap()).count();
            if count == 3 || count == 2 {
                HandType::FullHouse
            } else {
                HandType::FourOfAKind
            }
        } else {
            let mut count = vec.iter().filter(|&c| *c == *deduped.first().unwrap()).count();
            if count == 1 {
                count = vec.iter().filter(|&c| *c == *deduped.get(1).unwrap()).count();
            }
            if count == 1 {
                count = vec.iter().filter(|&c| *c == *deduped.get(2).unwrap()).count();
            }
            if count == 2 || count == 1 {
                HandType::TwoPair
            } else {
                HandType::ThreeOfAKind
            }
        }
    }
    
    fn hand_type(&self) -> HandType {
        let mut deduped = self.cards.to_vec();
        deduped.sort();
        deduped.dedup();
        let len = deduped.len();
        if len == 1 {
            HandType::FiveOfAKind
        } else if len == 5 {
            HandType::HighCard
        } else if len == 4 {
            HandType::OnePair
        } else if len == 2 {
            let count = self.cards.iter().filter(|&c| *c == *deduped.first().unwrap()).count();
            if count == 3 || count == 2 {
                HandType::FullHouse
            } else {
                HandType::FourOfAKind
            }
        } else {
            let mut count = self.cards.iter().filter(|&c| *c == *deduped.first().unwrap()).count();
            if count == 1 {
                count = self.cards.iter().filter(|&c| *c == *deduped.get(1).unwrap()).count();
            }
            if count == 1 {
                count = self.cards.iter().filter(|&c| *c == *deduped.get(2).unwrap()).count();
            }
            if count == 2 || count == 1 {
                HandType::TwoPair
            } else {
                HandType::ThreeOfAKind
            }
        }
    }

    fn hand_type_part2(&self) -> HandType {
        let copy = self.cards.to_vec();
        let mut map : HashMap<Card, i64> = HashMap::new();
        for card in copy.iter() {
            let curr = map.get(card);
            match curr {
                Some(x) => { map.insert(*card, x + 1); },
                None => { map.insert(*card, 1); }
            }
        }
        if !map.contains_key(&Card::Jack) {
            return self.hand_type()
        }
        map.remove(&Card::Jack);
        if map.is_empty() {
            return self.hand_type()
        }
        let top = map.iter().max_by(|(_k1,v1), (_k2,v2)| v1.cmp(v2)).map(|(k, _v)| k).unwrap();
        let mut new_hand : Vec<Card> = vec![];
        for card in &copy {
            match card {
                Card::Jack => { new_hand.push(*top) },
                _ => { new_hand.push(*card) }
            }
        }
        Hand::hand_type_from_vec(new_hand)
    }
}

impl Ord for Hand {
    // Replace hand_type_part2() with hand_type() for different part answers.. I know
    // not great.
    fn cmp(&self, other: &Self) -> Ordering {
        let value = self.hand_type_part2().cmp(&other.hand_type_part2());
        if value == Ordering::Equal {
            let mut indx = 0;
            let mut curr = Ordering::Equal;
            while curr == Ordering::Equal {
                curr = self.cards.get(indx).cmp(&other.cards.get(indx));
                indx += 1;
            }
            curr
        } else {
            value
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        true 
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Hand { }

fn main() {
    part1();
}

fn part1() {
    let contents = fs::read_to_string("inputs/input_7.txt").expect("Success");
    let mut hands : Vec<Hand> = vec![];
    for line in contents.lines() {
        let h = Hand::build_hand(line);
        hands.push(h);
    }
    hands.sort();

    let score = hands.iter().enumerate().fold(0, |acc : i64 , (ind, hand)| {
        acc + ((ind as i64 + 1) * hand.bid)
    });
    println!("Score: {}", score);
}
