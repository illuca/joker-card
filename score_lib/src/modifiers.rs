use ortalib::{ Edition, Enhancement };

use crate::{ explain, getters::EditionValueGetter, EnhancementValueGetter, Score };

/// Trait for applying Edition effects to Score
pub trait EditionUtils {
    /// Apply edition effects to score
    fn apply(&self, s: &mut Score) -> ();
}

impl EditionUtils for Option<Edition> {
    /// Apply edition effects if present
    fn apply(&self, s: &mut Score) -> () {
        match self {
            Some(e) => {
                // Get and apply edition values
                let (c, m, msg) = e.eidtion_value(s.mult);
                s.chips += c;
                s.mult += m;
                if msg == "" {
                    return;
                }
                explain!("  {:?} {} {:?}", e, msg, (s.chips, s.mult));
            }
            _ => (),
        }
    }
}

/// Trait for applying Enhancement effects to Score
pub trait EnhancementUtils {
    /// Apply enhancement effects to score
    fn apply(&self, s: &mut Score, is_held_in_hand: bool) -> ();
}

impl EnhancementUtils for Option<Enhancement> {
    /// Apply enhancement effects if present
    fn apply(&self, s: &mut Score, is_held_in_hand: bool) -> () {
        match self {
            Some(e) => {
                // Get and apply enhancement values
                let (c, m, msg) = e.enhancement_value(s.mult, is_held_in_hand);
                if msg == "" {
                    return;
                }
                s.chips += c;
                s.mult += m;
                explain!("  {:?} {} {:?}", e, msg, (s.chips, s.mult))
            }
            _ => (),
        }
    }
}
