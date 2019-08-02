#[macro_use]
extern crate error_chain;

pub use crate::card::Card;

#[allow(deprecated)]
pub mod errors {
    error_chain! {
         errors {
        UnrecognizedSuit(t: String) {
           description("Unrecognized suit")
           display("Unrecognized suit: {}", t)
        }
        UnrecognizedCardValue(t: String) {
           description("Unrecognized card value")
           display("Unrecognized card value: {}", t)
        }
    }

    foreign_links {

    }
        }
}

mod card;
