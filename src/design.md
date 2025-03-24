**Round** contains 3 vectors, cards_played, cards_held_in_hand, jokers.

First, we handle the cards_played, focusing on poker_hand_score.

We create a new file, called poker_hand_score.



poker_hand_score needs to handle the following problems:

* define a map H: poker_hand type -> (chips, mult)

* what the poker_hand it is

  * return poker_hand type

* according to the poker_hand type and the map H, we gets the chips C1 and mult. 

* the scored cards

  * For some poker_hand types, only parts of 5 cards count.

  * return a vector of scored cards

* according to the vector of scored cards, we gets the chips C2.

* At last, total chips is `(C1+C2) * mult`



## step2 card modifiers

* what the poker_hand it is

  * edge case: wild card(Is considered to be every suit simultaneously)
  * return poker_hand type

* according to the poker_hand type, we gets the basic chips C1 and mult.

* the scored cards

  * For some poker_hand types, only parts of 5 cards count.

  * return a vector of scored cards V

* Traverse the vector V, for each v in V, we consider the modifiers(card enhancement and edition), then we update he chips and mult.

* At last, total chips is `(C1+C2) * mult`





## Design

```
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub edition: Option<Edition>,
    #[doc(hidden)]
    unique_index: usize,
    // Skipped: Seal
}
```

Just follow the struct created above, we can create a struct called cards_played.

```rust
struct Score {
    // the poker hand type
    // the number of jokers
    // the cards played vector
    // the cards held in hand
}
impl poker_scores for PokerHand {
    
}
```

