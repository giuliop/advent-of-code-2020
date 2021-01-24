use std::collections::{HashSet, VecDeque};
use std::fs;

struct Deck {
    player: usize,
    cards: VecDeque<usize>,
}

impl Deck {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines();
        let player = lines.next().unwrap().split(" ").nth(1).unwrap()[..1]
            .parse::<usize>()
            .unwrap();
        let cards = lines.map(|n| n.parse::<usize>().unwrap()).collect();
        Deck { player, cards }
    }
}

fn play_game(player_1: &Deck, player_2: &Deck) -> Deck {
    let mut cards_1 = player_1.cards.clone();
    let mut cards_2 = player_2.cards.clone();
    loop {
        if cards_1.is_empty() {
            return Deck {
                player: 2,
                cards: cards_2,
            };
        }
        if cards_2.is_empty() {
            return Deck {
                player: 1,
                cards: cards_1,
            };
        }
        let card_1 = cards_1.pop_front().unwrap();
        let card_2 = cards_2.pop_front().unwrap();
        if card_1 > card_2 {
            cards_1.push_back(card_1);
            cards_1.push_back(card_2);
        } else {
            cards_2.push_back(card_2);
            cards_2.push_back(card_1);
        }
    }
}

fn play_recursive_game(player_1: &Deck, player_2: &Deck) -> Deck {
    let mut cards_1 = player_1.cards.clone();
    let mut cards_2 = player_2.cards.clone();
    let mut past_configurations = HashSet::new();

    loop {
        if !past_configurations.insert((cards_1.clone(), cards_2.clone())) {
            return Deck {
                player: 1,
                cards: cards_1,
            };
        }
        if cards_1.is_empty() {
            return Deck {
                player: 2,
                cards: cards_2,
            };
        }
        if cards_2.is_empty() {
            return Deck {
                player: 1,
                cards: cards_1,
            };
        }

        let card_1 = cards_1.pop_front().unwrap();
        let card_2 = cards_2.pop_front().unwrap();

        let winner: usize;
        if card_1 <= cards_1.len() && card_2 <= cards_2.len() {
            winner = play_recursive_game(
                &Deck {
                    player: 1,
                    cards: cards_1.iter().take(card_1).copied().collect(),
                },
                &Deck {
                    player: 2,
                    cards: cards_2.iter().take(card_2).copied().collect(),
                },
            )
            .player
        } else if card_1 > card_2 {
            winner = 1
        } else {
            winner = 2
        }

        //if card_1 > card_2 {
        if winner == 1 {
            cards_1.push_back(card_1);
            cards_1.push_back(card_2);
        } else {
            cards_2.push_back(card_2);
            cards_2.push_back(card_1);
        }
    }
}

fn calculate_score(deck: &Deck) -> usize {
    deck.cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i + 1))
        .sum::<usize>()
}

pub fn a() -> String {
    let decks = fs::read_to_string("../input/day22")
        .expect("Error reading file")
        .split("\n\n")
        .map(Deck::from_str)
        .collect::<Vec<Deck>>();

    let player_1 = &decks[0];
    let player_2 = &decks[1];

    let winning_deck: Deck = play_game(player_1, player_2);

    calculate_score(&winning_deck).to_string()
}

pub fn b() -> String {
    let decks = fs::read_to_string("../input/day22")
        .expect("Error reading file")
        .split("\n\n")
        .map(Deck::from_str)
        .collect::<Vec<Deck>>();

    let player_1 = &decks[0];
    let player_2 = &decks[1];

    let winning_deck: Deck = play_recursive_game(player_1, player_2);

    calculate_score(&winning_deck).to_string()
}
