use crate::cards::GenericCard;
use crate::ids::{AsId, Id};

#[derive(Debug)]
pub struct Collection<'a>(Vec<(GenericCard<'a>, usize)>) ;

impl<'gc> Collection<'gc>
{
    pub fn make<T>(items: Vec<T>) -> Collection<'gc>
        where T: TryInto<GenericCard<'gc>> + Clone
    {
        Collection(
            items.iter().map(
                |item|
                    {
                        if let Ok(i) = item.clone().try_into()
                        {
                            (i, 0usize)
                        } else {
                            panic!("failed.")
                        }

                    }
            ).collect()
        )
    }

    pub fn add<'any, T>(&mut self, id: T, quantity: usize)
        where T: AsId<'any>
    {
        let id: Id = id.as_id() ;
        for (card, amount) in self.0.iter_mut() {
            if id == card.as_id()
            {
                *amount += quantity ;
            }
        }
    }

    pub fn remove<'any, T>(&mut self, id: T, quantity: usize)
        where T: AsId<'any>
    {
        let id: Id = id.as_id() ;
        for (card, amount) in self.0.iter_mut() {
            if id == card.as_id()
            {
                *amount -= quantity ;
            }
        }
    }

    pub fn count<'any, T>(&self, id: T) -> Option<usize>
        where T: AsId<'any>
    {
        for (card, amount) in &self.0
        {
            if card.as_id() == id.as_id()
            {
                return Some(amount.clone())
            }
        }

        None
    }
}

pub struct Deck<'a>([Id<'a> ; 60]) ;