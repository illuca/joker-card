



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



## Easy Joker

For poker hand, it has two steps, one is recoginize. another is effect.

For modifiers, we have already know the type, what we should do is only effect.





Joker handing is divided into two parts, one is meet condition.

Another is effect.

for easy joker, we can just check if we meet the condition(poker_hand)

If we meet, we can handle the effect.





---



Since Flush take precedence over than straight, three of a kind...

Four finger flush might be three of a kind, two pair, straight.

why two pairs can't be seen as pair, why four finger flush can't be seen as a pair?

Because two pairs have 5 cards, 5 is not even. Flush also have 5 cards, 5 is not even.



 

for on scored joker, we pass a card to joker_value, then return the new chips and new mult.

