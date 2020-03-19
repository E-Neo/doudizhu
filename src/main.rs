use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug)]
enum PokerError {
    ParsePokerError,
}

impl std::fmt::Display for PokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerError::ParsePokerError => write!(f, "ParsePokerError"),
        }
    }
}

impl std::error::Error for PokerError {}

struct Poker {
    cards: u64,
}

impl Poker {
    fn new() -> Self {
        Poker { cards: 0 }
    }

    fn full() -> Self {
        let mut cards = 0;
        for _ in 0..54 {
            cards <<= 1;
            cards |= 1;
        }
        Poker { cards }
    }

    fn insert(&mut self, card: u8) -> bool {
        assert!(card >= 1 && card <= 15);
        if card < 14 {
            let shift = 4 * (card - 1);
            let bucket = (self.cards & (0b1111 << shift)) >> shift;
            if bucket == 0b0000 {
                self.cards |= 0b0001 << shift;
                true
            } else if bucket == 0b0001 {
                self.cards |= 0b0011 << shift;
                true
            } else if bucket == 0b0011 {
                self.cards |= 0b0111 << shift;
                true
            } else if bucket == 0b0111 {
                self.cards |= 0b1111 << shift;
                true
            } else {
                false
            }
        } else {
            let flag = 1 << 38 + card;
            if self.cards & flag == 0 {
                self.cards |= flag;
                true
            } else {
                false
            }
        }
    }

    fn remove(&mut self, card: u8) -> bool {
        assert!(card >= 1 && card <= 15);
        if card < 14 {
            let shift = 4 * (card - 1);
            let bucket = (self.cards & (0b1111 << shift)) >> shift;
            if bucket == 0b1111 {
                self.cards &= !(0b1000 << shift);
                true
            } else if bucket == 0b0111 {
                self.cards &= !(0b1100 << shift);
                true
            } else if bucket == 0b0011 {
                self.cards &= !(0b1110 << shift);
                true
            } else if bucket == 0b0001 {
                self.cards &= !(0b1111 << shift);
                true
            } else {
                false
            }
        } else {
            let flag = 1 << 38 + card;
            if self.cards & flag != 0 {
                self.cards &= !flag;
                true
            } else {
                false
            }
        }
    }

    fn remove_hand(&mut self, hand: &Self) -> bool {
        let old_cards = self.cards;
        for (card, count) in poker_to_hashmap(hand) {
            for _ in 0..count {
                if !self.remove(card) {
                    self.cards = old_cards;
                    return false;
                }
            }
        }
        true
    }

    fn count_card(&self, card: u8) -> u32 {
        assert!(card >= 1 && card <= 15);
        if card < 14 {
            let shift = 4 * (card - 1);
            let bucket = (self.cards & (0b1111 << shift)) >> shift;
            match bucket {
                0b0000 => 0,
                0b0001 => 1,
                0b0011 => 2,
                0b0111 => 3,
                _ => 4,
            }
        } else {
            let flag = 1 << 38 + card;
            if self.cards & flag == 0 {
                0
            } else {
                1
            }
        }
    }
}

impl std::str::FromStr for Poker {
    type Err = PokerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut poker = Poker::new();
        for x in s.chars().map(|c| c.to_ascii_uppercase()) {
            match x {
                '1'..='9' => {
                    if !poker.insert(x.to_digit(10).unwrap() as u8) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                '0' => {
                    if !poker.insert(10) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'A' => {
                    if !poker.insert(1) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'J' => {
                    if !poker.insert(11) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'Q' => {
                    if !poker.insert(12) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'K' => {
                    if !poker.insert(13) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'B' => {
                    if !poker.insert(14) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                'R' => {
                    if !poker.insert(15) {
                        return Err(PokerError::ParsePokerError);
                    }
                }
                _ => return Err(PokerError::ParsePokerError),
            }
        }
        Ok(poker)
    }
}

fn poker_to_hashmap(poker: &Poker) -> BTreeMap<u8, u32> {
    let mut map = BTreeMap::new();
    for c in 1..=15 {
        let count = poker.count_card(c);
        if count != 0 {
            map.insert(c, count);
        }
    }
    map
}

fn display_deck(deck: &Poker) {
    println!("R B 2 A K Q J 0 9 8 7 6 5 4 3");
    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        deck.count_card(15),
        deck.count_card(14),
        deck.count_card(2),
        deck.count_card(1),
        deck.count_card(13),
        deck.count_card(12),
        deck.count_card(11),
        deck.count_card(10),
        deck.count_card(9),
        deck.count_card(8),
        deck.count_card(7),
        deck.count_card(6),
        deck.count_card(5),
        deck.count_card(4),
        deck.count_card(3),
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Your Hand: ");
    std::io::stdout().flush()?;
    let mut deck = Poker::full();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if deck.remove_hand(&input.trim().parse()?) {
        display_deck(&deck);
        loop {
            print!("> ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            match &input.trim().parse() {
                Ok(hand) => {
                    if deck.remove_hand(&hand) {
                        display_deck(&deck);
                    } else {
                        println!("Wrong Hand");
                    }
                }
                Err(_) => println!("Wrong Hand"),
            }
        }
    } else {
        println!("Invalid Hand");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let deck = Poker::full();
        for c in 1..=13 {
            assert_eq!(deck.count_card(c), 4);
        }
        assert_eq!(deck.count_card(14), 1);
        assert_eq!(deck.count_card(15), 1);
    }

    #[test]
    fn test_insert() {
        let mut hand = Poker::new();
        for i in 1..=4 {
            assert_eq!(hand.insert(6), true);
            assert_eq!(hand.count_card(6), i);
        }
        assert_eq!(hand.insert(6), false);
        assert_eq!(hand.count_card(6), 4);
        assert_eq!(hand.insert(15), true);
        assert_eq!(hand.count_card(15), 1);
    }

    #[test]
    fn test_remove() {
        let mut deck = Poker::full();
        for i in (0..=3).rev() {
            assert_eq!(deck.remove(6), true);
            assert_eq!(deck.count_card(6), i);
        }
        assert_eq!(deck.remove(6), false);
        assert_eq!(deck.count_card(6), 0);
        assert_eq!(deck.remove(14), true);
        assert_eq!(deck.count_card(14), 0);
    }

    #[test]
    fn test_remove_hand() {
        let mut deck = Poker::full();
        assert_eq!(deck.remove_hand(&"KKKAAA00JJ".parse().unwrap()), true);
        assert_eq!(deck.count_card(13), 1);
        assert_eq!(deck.count_card(1), 1);
        assert_eq!(deck.count_card(10), 2);
        assert_eq!(deck.count_card(11), 2);
        assert_eq!(deck.remove_hand(&"KKKAAA00JJ".parse().unwrap()), false);
        assert_eq!(deck.count_card(13), 1);
        assert_eq!(deck.count_card(1), 1);
        assert_eq!(deck.count_card(10), 2);
        assert_eq!(deck.count_card(11), 2);
    }
}
