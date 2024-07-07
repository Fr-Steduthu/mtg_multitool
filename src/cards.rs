use crate::{Classification, ManaCost, Rarity};

#[derive(Debug, Clone)]
pub struct GenericCard<'a>
{
    id: Option<&'a str>,
    name: &'a str,

    cost: ManaCost,
    kind: Classification,
    rarity: Rarity,

    effects: &'a str,
}

impl<'s> GenericCard<'s>
{
    pub fn id(&self) -> crate::ids::Id
    {
        use crate::ids::AsId ;
        self.as_id()
    }

    pub fn name(&self) -> &&'s str
    {
        &self.name
    }
    pub fn cost(&self) -> &ManaCost
    {
        &self.cost
    }

    pub fn kind(&self) -> &Classification
    {
        &self.kind
    }

    pub fn rarity(&self) -> &Rarity
    {
        &self.rarity
    }

    pub fn effects(&self) -> &&'s str
    {
        &self.effects
    }
}

impl<'s> TryFrom<&'s str> for GenericCard<'s>
{
    type Error = &'static str;

    fn try_from(value: &'s str) -> Result<Self, Self::Error>
    {

        let mut iter = value.split(";").map(str::trim) ;

        let id_str = iter.next() ;
        if let Some(name_str) = iter.next()
        {
            if let Some(cost_str) = iter.next()
            {
                let cost_conversion_result = ManaCost::try_from(cost_str) ;
                if let Ok(cost) = cost_conversion_result
                {
                    if let Some(kind_str) = iter.next()
                    {
                        if let Some(rarity_str) = iter.next()
                        {
                            let rarity_conversion_result = Rarity::try_from(rarity_str) ;

                            return if let Ok(rarity) = rarity_conversion_result
                            {
                                let kind_conversion_result = Classification::try_from(kind_str);

                                if kind_conversion_result.is_ok()
                                {
                                    return if let Some(effects) = iter.next()
                                    {
                                        if iter.next().is_some()
                                        {
                                            Err("Too many fields")
                                        } else {
                                            Ok(
                                                GenericCard
                                                {
                                                    id: id_str,
                                                    name: name_str,
                                                    cost,
                                                    kind: kind_conversion_result.unwrap(),
                                                    rarity,
                                                    effects,
                                                }
                                            )
                                        }
                                    } else {
                                        Err("No effect given")
                                    }
                                } else {
                                    Err(kind_conversion_result.err().unwrap())
                                }

                            } else {
                                Err(rarity_conversion_result.err().unwrap())
                            }
                        }
                        return Err("No rarity level found") ;
                    }
                }

                return Err(cost_conversion_result.err().unwrap()) ;
            }

            return Err("No Cost data found") ;
        }

        return Err("No Name given") ;
    }
}

#[test]
fn genericcard_try_from_str()
{
    todo!()
}

impl<'a> From<GenericCard<'a>> for crate::ids::Id<'a>
{
    fn from(value: GenericCard<'a>) -> Self
    {
        use crate::ids::Id::{Both, Name} ;
        if let Some(id) = value.id
        {
            return Both(value.name, id) ;
        }

        return Name(value.name) ;
    }
}

#[test]
fn id_from_generic_card()
{
    todo!()
}