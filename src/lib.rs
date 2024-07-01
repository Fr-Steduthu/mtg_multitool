use std::str::FromStr;
use crate::Id::{Both, Name, Serial};

/// Examples:
/// `Serial("LTR C 0001")`
/// `Name("Banish from edoras")`
/// `Both("Banish from Edoras", "LTR C 0001")`
#[derive(Debug)]
pub enum Id<'s>
{
    Name(&'s str),
    Serial(&'s str),
    Both(&'s str, &'s str),
}

impl PartialEq for Id<'_>
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            Name(n) => if let Name(nn) = other { n == nn } else { false },
            Serial(s) => if let Serial(ss) = other { s == ss } else { false },
            Both(n, s) => {
                match other
                {
                    Name(nn) => n == nn,
                    Serial(ss) => s == ss,
                    Both(nn, ss) => n == nn || s == ss,
                }
            }
        }
    }
}

fn is_serial(s: &str) -> bool
{
    let mut ss = s.split(" ") ;
    if let Some(_) = ss.next()
    {
        if let Some(rarity) = ss.next()
        {
            if let Ok(_) = Rarity::try_from(rarity)
            {
                if let Some(id) = ss.next()
                {
                    if let Ok(_) = id.parse::<usize>()
                    {
                        return true ;
                    }
                }
            }
        }
    }

    false
}
impl<'s> From<&'s str> for Id<'s>
{
    fn from(value: &'s str) -> Self {
        if is_serial(value) { return Serial(value); }
        else { Name(value) }
    }
}


/// Common / Uncommon / Rare / Mythical
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rarity
{
    Common,
    Uncommon,
    Rare,
    Mythical,
}
impl<'a> TryFrom<&'a str> for Rarity
{
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(match value.trim().to_ascii_uppercase().as_str()
        {
            "C" => Rarity::Common,
            "U" => Rarity::Uncommon,
            "R" => Rarity::Rare,
            "M" => Rarity::Mythical,

            &_ => return Err("Could not parse into rarity"),
        })
    }
}

#[test]
fn rarity_from_str()
{
    use Rarity::{*} ;
    assert_eq!(Rarity::try_from(" c ").expect("\" c \" test failed"), Common) ;
    assert_eq!(Rarity::try_from("U ").expect("\"U \" test failed"), Uncommon) ;
    assert_eq!(Rarity::try_from("  R").expect("\"  R\" test failed"), Rare) ;
    assert_eq!(Rarity::try_from("m").expect("\"m\" test failed"), Mythical) ;

}


/// Represents:
/// `"Token [...]"`
/// `"Legendary Creature - [...]"`
/// `"Creature - [...]"`
///
/// `"Artifact - [...]"`
///
/// `"Sorcery"`
/// `"Ritual"`
/// `"Enchantment"`
///
/// `"Basic Land - [...]"`
/// `"Legendary Land"`
#[derive(Debug, Clone, PartialEq)]
pub enum Classification
{
    /// `"Token [...]"`
    Token(Box<Classification>),

    /// `"Creature - [...]"`
    Creature(&'static str),
    /// `"Legendary Creature - [...]"`
    LegendaryCreature(&'static str),

    /// `"Sorcery"`
    Sorcery,
    /// `"Ritual"`
    Ritual,
    /// `"Enchantment"`
    Enchantment,

    /// `"Artifact - [...]"`
    Artifact(&'static str),

    /// `"Basic Land - [...]"`
    Terrain,
    /// `"Legendary Land"`
    LegendaryTerrain,
}

impl TryFrom<&'static str> for Classification
{
    type Error = &'static str;

    fn try_from(s: &'static str) -> Result<Self, Self::Error> {
        /*use crate::Classification::{*} ;

        let mut s = s.to_ascii_lowercase().split("-") ;

        if s.count() != 1
        {
            if let Some(first_word) = s.next()
            {
                match first_word.trim()
                {
                    str if &str[0..5] == "token" => return Ok(Token(Box::new(Classification::from_str(s.collect())?))),
                    "creature" => return
                        &_ => {}
                }
            }
        }*/
        unimplemented!("TryFrom<&'static str> for Classification") ;
    }
}

#[test]
fn test_class_from_str()
{
    todo!()
}



/// ManaCost(colorless, white, blue, black, red, green)
#[derive(Debug, Clone,Copy)]
pub struct ManaCost(u8, u8, u8, u8, u8, u8) ;

impl ManaCost
{
    pub fn colorless(&self) -> u8
    { self.0 }
    pub fn white(&self) -> u8
    { self.1 }
    pub fn blue(&self) -> u8
    { self.2 }
    pub fn black(&self) -> u8
    { self.3 }
    pub fn red(&self) -> u8
    { self.4 }
    pub fn green(&self) -> u8
    { self.5 }
}

impl Default for ManaCost
{
    fn default() -> Self {
        ManaCost(0,0,0,0,0,0)
    }
}

impl TryFrom<&'_ str> for ManaCost
{
    type Error = &'static str;

    /// Converts str `n {"w" | "blu" | "bla" | "r" | "g"}` with n >= 0 to a ManaCost data struct
    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        if value.trim() == "" { return Err("str is empty") ; }

        let mut result = ManaCost::default() ;

        let mut iter = value.trim().split(" ") ;

        let f =
            {
                if let Some(v) = iter.next()
                {
                    v
                } else {
                    return Err("No colorless cost given")
                }
            }
            ;

        if let Ok(i) = u8::from_str(f.to_ascii_lowercase().as_str())
        {
            result.0 = i ;
        }

        for str in iter {
            match str.to_ascii_lowercase().as_str()
            {
                "w" | "white" => result.1 += 1,
                "blu" | "blue" => result.2 += 1,
                "bla" | "black" => result.3 += 1,
                "r" | "red" => result.4 += 1,
                "g" | "green" => result.5 += 1,
                &_ => return Err("An error has occurred while parsing str to ManaCost object.")
            }
        }

        Ok(result)
    }
}

pub struct GenericCard<'a>
{
    id: Option<&'a str>,
    name: &'a str,

    cost: ManaCost,
    kind: Classification,

    effects: &'a str,
}

impl<'s> TryFrom<&'s str> for GenericCard<'s>
{
    type Error = &'static str;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> From<GenericCard<'a>> for Id<'a>
{
    fn from(value: GenericCard<'a>) -> Self {
        if let Some(id) = value.id
        {
            return Both(value.name, id) ;
        }

        return Name(value.name) ;
    }
}

pub struct Collection<'a>(Vec<(GenericCard<'a>, usize)>) ;

impl<'a> Collection<'a>
{
    pub fn make<T: TryInto<GenericCard<'a>> + Clone>(items: Vec<T>) -> Collection<'a>
    {
        //todo retirer le .zip
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

    pub fn add<T: Into<Id<'a>>>(&mut self, id: T)
    {
        todo!()
    }

    pub fn remove<T: Into<Id<'a>>>(&mut self, id: T)
    {
        todo!()
    }
}

pub struct Deck<'a>([Id<'a> ; 60]) ;

include!("../assets/ltr/mod.rs");

/*#[test]
fn esg_lua() -> Result<(), &'static str>
{
    let mut lua = ltr::collection() ;

    lua.add(&"LTR C 000000001", 1)? ;
    lua.add(&"Banish from Edoras", 1)? ;
    lua.add(&"LTR C 0001", 1)? ;
    lua.add(&"LTR C 1", 1)? ;
    lua.add(&ltr::BANISH_FROM_EDORAS(), 1)? ;

    lua.add(&"LTR R 2", 2) ? ;
    lua.add(&"The battle of Bywater", 2) ? ;

    println!("{lua:?}") ;

    Ok(())
}*/