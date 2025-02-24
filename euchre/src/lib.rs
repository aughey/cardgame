use rand::seq::SliceRandom;

pub mod card;
pub mod euchre;

pub fn shuffle<T>(deck: &mut [T]) -> &mut [T] {
    deck.shuffle(&mut rand::rng());
    deck
}

/// Creates an unshuffled standard euchre deck 9, 10, J, Q, K, A of each suit.
pub fn euchre_deck() -> Vec<card::Card> {
    let seven_through_10 = (9..=10)
        .flat_map(|n| card::Suit::iter().map(move |suit| card::Card::new(suit, n).unwrap()));
    let face_cards = card::Special::iter().flat_map(|special| {
        card::Suit::iter().map(move |suit| card::Card::new_special(suit, special.clone()))
    });

    seven_through_10.chain(face_cards).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euchre_deck() {
        let deck = euchre_deck();
        assert_eq!(deck.len(), 5 * 4 + 4);
    }
    #[test]
    fn test_shuffle() {
        let mut deck = euchre_deck();
        let original = deck.clone();
        shuffle(deck.as_mut_slice());
        assert_ne!(deck, original);
    }
}
