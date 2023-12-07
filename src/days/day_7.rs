use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventResult};
use std::cmp::Reverse;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut hands = parser::as_vec_by_line(&mut input_file, line_parser)?;
    hands.sort_unstable();

    // Part 1
    utils::part_header(1);
    part_1(&hands);

    Ok(())
}

// Assumes incoming slice is sorted
fn part_1(hands: &[Hand]) {
    let total_winnings: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum();

    println!("Total winnings: {total_winnings}");
}

// Ordered by ascending value
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err("Invalid character for card"),
        }
    }
}

// Ordered by ascending value
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<[Card; 5]> for HandType {
    fn from(mut cards: [Card; 5]) -> Self {
        cards.sort_unstable();
        let mut groups: [u8; 5] = [0, 0, 0, 0, 0];
        let mut i = 0;
        let mut current_card = cards[0];
        for card in cards {
            if card != current_card {
                current_card = card;
                i += 1;
            }

            groups[i] += 1
        }

        groups.sort_unstable_by_key(|&x| Reverse(x));

        match groups[0] {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 => match groups[1] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeKind,
            },
            2 => match groups[1] {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }
}

// Note: Ordering of these fields is important.
// It defines lexicographic ordering for Ord/PartialOrd.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: u64,
}

fn line_parser(line: &str) -> AdventResult<Hand> {
    let Some((cards_str, bid)) = line.split_once(' ') else {
        return Err(InputParse(format!("Failed to split line {line}")));
    };

    if cards_str.len() > 5 {
        return Err(InputParse(format!(
            "Cards string is more than 5 characters: '{cards_str}"
        )));
    }

    let Ok(bid) = bid.parse() else {
        return Err(InputParse(format!("Failed to parse bid '{bid}' to u64")));
    };

    use crate::days::day_7::Card::Two;
    let mut cards: [Card; 5] = [Two, Two, Two, Two, Two];
    for (i, card) in cards_str.trim().chars().enumerate() {
        cards[i] = card
            .try_into()
            .map_err(|s: &str| InputParse(s.to_string()))?;
    }

    let hand_type = cards.into();

    Ok(Hand {
        cards,
        bid,
        hand_type,
    })
}
