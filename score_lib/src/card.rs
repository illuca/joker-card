use ortalib::{ Card, Enhancement };

pub trait CardUtils {
    fn is_wild(&self) -> bool;
}
impl CardUtils for Card {
    fn is_wild(&self) -> bool {
        match self.enhancement {
            Some(e) => e == Enhancement::Wild,
            _ => false,
        }
    }
}
