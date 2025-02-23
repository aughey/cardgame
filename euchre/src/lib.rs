pub mod card;
use card::{Card, Suit};

pub struct TeamPlay<'a> {
    pub player0: Option<&'a Card>,
    pub player1: Option<&'a Card>,
}

pub fn best_optional_card_lr(
    solver: &impl CardSolver,
    left: Option<&Card>,
    right: Option<&Card>,
) -> Option<LeftRight> {
    match (left, right) {
        (Some(a), Some(b)) => match solver.test(a, b) {
            Some(lr) => Some(lr),
            None => None,
        },
        (Some(_), None) => Some(LeftRight::Left),
        (None, Some(_)) => Some(LeftRight::Right),
        (None, None) => None,
    }
}

pub fn best_optional_card<'a>(
    solver: &impl CardSolver,
    left: Option<&'a Card>,
    right: Option<&'a Card>,
) -> Option<&'a Card> {
    match best_optional_card_lr(solver, left, right) {
        Some(LeftRight::Left) => left,
        Some(LeftRight::Right) => right,
        None => None,
    }
}

pub fn winning_team(
    solver: &impl CardSolver,
    left: &TeamPlay,
    right: &TeamPlay,
) -> Option<LeftRight> {
    let best_left = best_optional_card(solver, left.player0, left.player1);
    let best_right = best_optional_card(solver, right.player0, right.player1);
    best_optional_card_lr(solver, best_left, best_right)
}

#[derive(Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

pub trait CardSolver {
    fn test(&self, card0: &Card, card1: &Card) -> Option<LeftRight>;
}

pub struct HandParams<'a> {
    lead: &'a Card,
    trump: Suit,
}

impl<'a> HandParams<'a> {
    /// The suit leading the hand.  This is not necessarily the suit of the lead card.
    pub fn suit_lead(&self) -> &Suit {
        if self.lead.is_jack() && self.lead.suit().opposite_color() == self.trump {
            return &self.trump;
        } else {
            return self.lead.suit();
        }
    }

    pub fn is_right_bower(&self, card: &Card) -> bool {
        card.is_jack() && card.suit() == &self.trump
    }

    pub fn is_left_bower(&self, card: &Card) -> bool {
        card.is_jack() && card.suit() == &self.trump.opposite_color()
    }
}

impl<'a> CardSolver for HandParams<'a> {
    fn test(&self, left_card: &Card, right_card: &Card) -> Option<LeftRight> {
        // Check for right or left bowers
        if self.is_right_bower(left_card) {
            return Some(LeftRight::Left);
        }
        if self.is_right_bower(right_card) {
            return Some(LeftRight::Right);
        }
        if self.is_left_bower(left_card) {
            return Some(LeftRight::Left);
        }
        if self.is_left_bower(right_card) {
            return Some(LeftRight::Right);
        }

        // Check for trump
        match (
            left_card.suit() == &self.trump,
            right_card.suit() == &self.trump,
        ) {
            (true, true) => {
                // Both cards are trump, compare ranks (no jacks)
                if left_card.rank() > right_card.rank() {
                    return Some(LeftRight::Left);
                } else if left_card.rank() < right_card.rank() {
                    return Some(LeftRight::Right);
                } else {
                    // Cannot have equal ranks and suits, but we will say neither wins
                    return None;
                }
            }
            // One is trump, it's that one
            (true, false) => return Some(LeftRight::Left),
            (false, true) => return Some(LeftRight::Right),
            // Neither trump, check for following suit
            (false, false) => {}
        }

        // Check for following suit
        match (
            self.suit_lead() == left_card.suit(),
            self.suit_lead() == right_card.suit(),
        ) {
            // Both cards follow suit
            // We have no jacks to worry about, so it's just rank.
            (true, true) => {
                if left_card.rank() > right_card.rank() {
                    Some(LeftRight::Left)
                } else {
                    Some(LeftRight::Right)
                }
            }
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
        // Hearts is trump, leading with hearts.
        let solver = HandParams {
            lead: &Card::new(Suit::Hearts, 10).unwrap(),
            trump: Suit::Hearts,
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

        // Right Bower wins over ace
        assert_eq!(
            solver.test(
                &Card::new_special(Suit::Hearts, card::Special::Jack),
                &Card::new_special(Suit::Hearts, card::Special::Ace)
            ),
            Some(LeftRight::Left)
        );

        // Left Bower wins over ace
        assert_eq!(
            solver.test(
                &Card::new_special(Suit::Diamonds, card::Special::Jack),
                &Card::new_special(Suit::Hearts, card::Special::Ace)
            ),
            Some(LeftRight::Left)
        );

        // 2 of trump wins over non-trump
        assert_eq!(
            solver.test(
                &Card::new(Suit::Hearts, 2).unwrap(),
                &Card::new_special(Suit::Spades, card::Special::Jack),
            ),
            Some(LeftRight::Left)
        );

        // If neither follows suit, then neither wins.
        assert_eq!(
            solver.test(
                &Card::new(Suit::Diamonds, 10).unwrap(),
                &Card::new(Suit::Spades, 10).unwrap()
            ),
            None
        );
    }

    #[test]
    fn test_hand_solve() {
        let player0 = Card::new(Suit::Hearts, 9).unwrap();
        let player1 = Card::new(Suit::Diamonds, 9).unwrap();
        let player2 = Card::new(Suit::Spades, 9).unwrap();
        let player3 = Card::new(Suit::Clubs, 9).unwrap();

        let team0 = TeamPlay {
            player0: Some(&player0),
            player1: Some(&player1),
        };
        let team1 = TeamPlay {
            player0: Some(&player2),
            player1: Some(&player3),
        };

        let solver = HandParams {
            lead: team0.player0.as_ref().unwrap(),
            trump: Suit::Hearts,
        };

        // team0 (left) wins
        assert_eq!(winning_team(&solver, &team0, &team1), Some(LeftRight::Left));
    }
}
