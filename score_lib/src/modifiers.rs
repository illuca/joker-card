use ortalib::{ Edition, Enhancement };

use crate::{ explain, getters::EditionValueGetter, EnhancementValueGetter, Score };

pub trait EditionUtils {
    fn apply(&self, s: &mut Score) -> ();
}

impl EditionUtils for Option<Edition> {
    fn apply(&self, s: &mut Score) -> () {
        match self {
            Some(e) => {
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
pub trait EnhancementUtils {
    fn apply(&self, s: &mut Score, is_held_in_hand: bool) -> ();
}
impl EnhancementUtils for Option<Enhancement> {
    fn apply(&self, s: &mut Score, is_held_in_hand: bool) -> () {
        match self {
            Some(e) => {
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
