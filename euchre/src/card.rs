use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Suit {
    pub fn opposite_color(&self) -> Suit {
        match self {
            Suit::Spades => Suit::Clubs,
            Suit::Hearts => Suit::Diamonds,
            Suit::Diamonds => Suit::Hearts,
            Suit::Clubs => Suit::Spades,
        }
    }
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs].into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Special {
    Jack,
    Queen,
    King,
    Ace,
}
impl Special {
    pub fn iter() -> impl Iterator<Item = Special> {
        [Special::Jack, Special::Queen, Special::King, Special::Ace].into_iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: u8,
}
impl Card {
    pub fn new_special(suit: Suit, special: Special) -> Self {
        // Bower is jack and is always 11
        let rank = match special {
            Special::Jack => 11,
            Special::Queen => 12,
            Special::King => 13,
            Special::Ace => 14,
        };
        Card { suit, rank }
    }
    /// Rank must be between 2 and 10.  Face cards and ace are special.
    pub fn new(suit: Suit, rank: u8) -> Result<Self> {
        if rank < 2 || rank > 10 {
            return Err(anyhow::anyhow!("Invalid rank: {}", rank));
        }
        Ok(Card { suit, rank })
    }

    pub fn is_jack(&self) -> bool {
        self.rank == 11
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(Suit::Hearts, 10).unwrap();
        assert_eq!(card.suit(), &Suit::Hearts);
        assert_eq!(card.rank(), 10);
    }

    #[test]
    fn test_invalid_rank() {
        let result = Card::new(Suit::Spades, 15);
        assert!(result.is_err());
    }
}
