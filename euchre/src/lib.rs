pub mod card;
use card::{Card, Suit};

pub struct TeamPlay {
    pub player0: Option<Card>,
    pub player1: Option<Card>,
}
impl TeamPlay {
    pub fn highest_card(&self, solver: &impl CardSolver) -> Option<Card> {
        match (self.player0.as_ref(), self.player1.as_ref()) {
            (None, None) => None,
            (None, Some(b)) => Some(b.clone()),
            (Some(a), None) => Some(a.clone()),
            (Some(a), Some(b)) => match solver.test(a, b) {
                Some(lr) => match lr {
                    LeftRight::Left => Some(a.clone()),
                    LeftRight::Right => Some(b.clone()),
                },
                None => None,
            },
        }
    }
}

pub trait HandSolver {
    fn test(&self, team0: &TeamPlay, team1: &TeamPlay) -> LeftRight;
}

#[derive(Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

pub trait CardSolver {
    fn test(&self, card0: &Card, card1: &Card) -> Option<LeftRight>;
}

pub struct HandParams {
    pub lead: Card,
}
impl HandParams {
    pub fn suit_of_card_played(&self, card: &Card) -> Suit {
        // If the card is the jack of the opposite bower, then it is our suit
        if card.is_jack() && card.suit() == &self.lead.suit().opposite_color() {
            self.suit().clone()
        } else {
            card.suit().clone()
        }
    }
    fn suit(&self) -> &Suit {
        self.lead.suit()
    }
}

impl CardSolver for HandParams {
    fn test(&self, left_card: &Card, right_card: &Card) -> Option<LeftRight> {
        let left_suit = self.suit_of_card_played(left_card);
        let right_suit = self.suit_of_card_played(right_card);
        // Check for following suit
        match (
            self.lead.suit() == &left_suit,
            self.lead.suit() == &right_suit,
        ) {
            // Both cards follow suit
            // Check for jacks
            (true, true) => match (left_card.is_jack(), right_card.is_jack()) {
                (true, true) => {
                    // Both jacks, the same suit wins
                    if left_card.suit() == self.suit() {
                        Some(LeftRight::Left)
                    } else {
                        Some(LeftRight::Right)
                    }
                }
                (true, false) => Some(LeftRight::Left),
                (false, true) => Some(LeftRight::Right),
                (false, false) => {
                    // Neither are jacks, compare ranks
                    if left_card.rank() > right_card.rank() {
                        Some(LeftRight::Left)
                    } else if left_card.rank() < right_card.rank() {
                        Some(LeftRight::Right)
                    } else {
                        // Cannot have equal ranks and suits, but we will say neither wins
                        None
                    }
                }
            },
            // Only one or none follow suit, easy
            (true, false) => Some(LeftRight::Left),
            (false, true) => Some(LeftRight::Right),
            (false, false) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use card::Suit;

    #[test]
    fn test_highest_card() {
        let solver = HandParams {
            lead: Card::new(Suit::Hearts, 10).unwrap(),
        };

        // Follow suit, left wins
        assert_eq!(
            solver.test(
                &Card::new(Suit::Hearts, 10).unwrap(),
                &Card::new(Suit::Hearts, 9).unwrap()
            ),
            Some(LeftRight::Left)
        );
        // Follow suit, right wins
        assert_eq!(
            solver.test(
                &Card::new(Suit::Hearts, 9).unwrap(),
                &Card::new(Suit::Hearts, 10).unwrap()
            ),
            Some(LeftRight::Right)
        );

        // One side doesn't follow suit, left wins
        assert_eq!(
            solver.test(
                &Card::new(Suit::Hearts, 9).unwrap(),
                &Card::new(Suit::Diamonds, 10).unwrap()
            ),
            Some(LeftRight::Left)
        );

        // Ace high
        assert_eq!(
            solver.test(
                &Card::new_special(Suit::Hearts, card::Special::Ace),
                &Card::new(Suit::Hearts, 10).unwrap()
            ),
            Some(LeftRight::Left)
        );

        // Bower wins over ace
        assert_eq!(
            solver.test(
                &Card::new_special(Suit::Hearts, card::Special::Jack),
                &Card::new_special(Suit::Hearts, card::Special::Ace)
            ),
            Some(LeftRight::Left)
        );
    }
}
