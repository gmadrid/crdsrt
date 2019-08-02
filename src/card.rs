use itertools::Itertools;

use crate::errors::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum CardValue {
    VA,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VT,
    VJ,
    VQ,
    VK,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum CardSuit {
    Clubs,
    Hearts,
    Spades,
    Diamonds,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Card {
    suit: CardSuit,
    val: CardValue,
}

impl Card {
    pub fn new(val: CardValue, suit: CardSuit) -> Card {
        Card { suit, val }
    }

    pub fn parse_vec(s: &str) -> Result<Vec<Card>> {
        Card::parse_vec_pat(s, ",")
    }

    pub fn parse_vec_pat(s: &str, pat: &str) -> Result<Vec<Card>> {
        s.split(pat)
            .map(|piece| piece.trim())
            .map(|trimmed| Card::parse(trimmed))
            .fold_results(Vec::new(), |mut vec, crd| {vec.push(crd.0); vec })
    }

    pub fn parse(s: &str) -> Result<(Card, &str)> {
        // Grammar.
        // Note: no white space is allowed before, after, or inside the card string.
        //
        // Value: 'A' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'T' | 'J' | 'Q' | 'K' | '10'
        // Suit: 'C' | 'H' | 'S' | 'D'
        // Card: Value+Suit
        let (val, rest) = Card::read_value(&s)?;
        let (suit, rest) = Card::read_suit(rest)?;

        Ok((Card::new(val, suit), rest))
    }

    fn read_value(s: &str) -> Result<(CardValue, &str)> {
        let mut start = 1;
        let mut chars = s.chars();
        let value = match chars.next() {
            Some('A') => Option::Some(CardValue::VA),
            Some('2') => Option::Some(CardValue::V2),
            Some('3') => Option::Some(CardValue::V3),
            Some('4') => Option::Some(CardValue::V4),
            Some('5') => Option::Some(CardValue::V5),
            Some('6') => Option::Some(CardValue::V6),
            Some('7') => Option::Some(CardValue::V7),
            Some('8') => Option::Some(CardValue::V8),
            Some('9') => Option::Some(CardValue::V9),
            Some('T') => Option::Some(CardValue::VT),
            Some('J') => Option::Some(CardValue::VJ),
            Some('Q') => Option::Some(CardValue::VQ),
            Some('K') => Option::Some(CardValue::VK),
            Some('1') => {
                if chars.next() == Some('0') {
                    start = 2;
                    Option::Some(CardValue::VT)
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        };

        let tuple = value.map(|v| (v, &s[start..]));
        tuple.ok_or_else(|| ErrorKind::UnrecognizedCardValue(s.to_owned()).into())
    }

    fn read_suit(str: &str) -> Result<(CardSuit, &str)> {
        let suit = match str.chars().next() {
            Some('S') => Option::Some(CardSuit::Spades),
            Some('H') => Option::Some(CardSuit::Hearts),
            Some('D') => Option::Some(CardSuit::Diamonds),
            Some('C') => Option::Some(CardSuit::Clubs),
            _ => Option::None,
        };

        let tuple = suit.map(|s| (s, &str[1..]));
        tuple.ok_or_else(|| ErrorKind::UnrecognizedSuit(str.to_owned()).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ord() {
        let ace_s = Card::new(CardValue::VA, CardSuit::Spades);
        let two_s = Card::new(CardValue::V2, CardSuit::Spades);
        assert!(ace_s < two_s);
        assert_eq!(ace_s, ace_s);
        assert_eq!(two_s, two_s);
    }

    #[test]
    fn test_parse_suit() {
        assert_eq!((CardSuit::Spades, "3"), Card::read_suit("S3").unwrap());
        assert_eq!((CardSuit::Hearts, "4"), Card::read_suit("H4").unwrap());
        assert_eq!((CardSuit::Diamonds, "5"), Card::read_suit("D5").unwrap());
        assert_eq!((CardSuit::Clubs, "6"), Card::read_suit("C6").unwrap());
        assert!(Card::read_suit("X").is_err());
        assert!(Card::read_suit("").is_err());
    }

    #[test]
    fn test_parse_value() {
        assert_eq!((CardValue::VA, "X"), Card::read_value("AX").unwrap());
        assert_eq!((CardValue::V2, "X"), Card::read_value("2X").unwrap());
        assert_eq!((CardValue::V3, "X"), Card::read_value("3X").unwrap());
        assert_eq!((CardValue::V4, "X"), Card::read_value("4X").unwrap());
        assert_eq!((CardValue::V5, "X"), Card::read_value("5X").unwrap());
        assert_eq!((CardValue::V6, "X"), Card::read_value("6X").unwrap());
        assert_eq!((CardValue::V7, "X"), Card::read_value("7X").unwrap());
        assert_eq!((CardValue::V8, "X"), Card::read_value("8X").unwrap());
        assert_eq!((CardValue::V9, "X"), Card::read_value("9X").unwrap());
        assert_eq!((CardValue::VT, "X"), Card::read_value("TX").unwrap());
        assert_eq!((CardValue::VJ, "X"), Card::read_value("JX").unwrap());
        assert_eq!((CardValue::VQ, "X"), Card::read_value("QX").unwrap());
        assert_eq!((CardValue::VK, "X"), Card::read_value("KX").unwrap());

        assert_eq!((CardValue::VT, "X"), Card::read_value("10X").unwrap());

        assert!(Card::read_value("XX").is_err());
        assert!(Card::read_value("11").is_err());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            (Card::new(CardValue::VA, CardSuit::Spades), "REST"),
            Card::parse("ASREST").unwrap()
        );
        assert_eq!(
            (Card::new(CardValue::V2, CardSuit::Spades), "REST"),
            Card::parse("2SREST").unwrap()
        );
        assert_eq!(
            (Card::new(CardValue::VA, CardSuit::Hearts), "REST"),
            Card::parse("AHREST").unwrap()
        );
        assert_eq!(
            (Card::new(CardValue::V8, CardSuit::Clubs), "REST"),
            Card::parse("8CREST").unwrap()
        );

        assert_eq!(
            (Card::new(CardValue::VT, CardSuit::Spades), "REST"),
            Card::parse("10SREST").unwrap()
        );
    }
}
