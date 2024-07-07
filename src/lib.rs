use std::str::FromStr;
use crate::Classification::{Artifact, Creature, LegendaryCreature, Sorcery, Token};

pub mod ids;
pub mod cards;
pub mod collections;

/// Represents an MTG Card's rarity (Common / Uncommon / Rare / Mythical)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Rarity
{
    Common,
    Uncommon,
    Rare,
    Mythical,
}

/// ```
/// use mtg_multitool::Rarity;
/// assert_eq!(Rarity::try_from("c"), Ok(Rarity::Common)) ;
/// assert_eq!(Rarity::try_from("u"), Ok(Rarity::Uncommon)) ;
/// assert_eq!(Rarity::try_from("r"), Ok(Rarity::Rare)) ;
/// assert_eq!(Rarity::try_from("m"), Ok(Rarity::Mythical)) ;
/// ```
impl<'a> TryFrom<&'a str> for Rarity
{
    type Error = &'static str;

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
fn rarity_try_from_str()
{
    use crate::Rarity;
    assert_eq!(Rarity::try_from(" c "), Ok(Rarity::Common)) ;
    assert_eq!(Rarity::try_from("U  "), Ok(Rarity::Uncommon)) ;
    assert_eq!(Rarity::try_from("  R"), Ok(Rarity::Rare)) ;
    assert_eq!(Rarity::try_from("m"), Ok(Rarity::Mythical)) ;
}


/// Represents a card's kind, such as `"Legendary Land"` or `"Sorcery"`
#[derive(Debug, Clone)]
pub enum Classification<'mostly_static>
{
    /// Sorcery (`"Sorcery"`)
    Sorcery,
    /// Rituals (`"Ritual"`)
    Ritual,
    /// Enchantments (`"Enchantment"`)
    Enchantment,
    /// Lands (`"Land"`)
    Terrain,
    /// Regular artifacts (`"Artifact"`)
    Artifact,

    /// Artifact creatures (`"Artifact creature"`)
    //todo,

    /// Creatures (`"Creature - [...]"`)
    Creature(&'mostly_static str),

    /// `"Token [...]"`
    Token(Box<Classification<'mostly_static>>),
    /// `"Legendary [...]"`
    Legendary(Box<Classification<'mostly_static>>),
}

/// The recognized str are:
/// <ul>
/// <li>`"Sorcery"`</li>
/// <li>`"Ritual"`</li>
/// <li>`"Enchantment"`</li>
/// <li>`"Basic Land"`</li>
/// <li>`"Artifact"`</li>
///
/// <li>`"Creature - [...]"`</li>
///
/// <li>`"Token [...]"`</li>
/// <li>`"Legendary [...]"`</li>
/// </ul>
impl<'mostly_static> TryFrom<&'mostly_static str> for Classification<'mostly_static>
{
    type Error = &'static str;

    fn try_from(s: &'mostly_static str) -> Result<Self, Self::Error>
    {
        let s = match s.trim().to_ascii_lowercase().as_str()
        {
            "sorcery" => return false,
            "ritual" => return false,
            "enchantment" => return false,
            "land" => return false,
            "artifact" => return false,

            other => other,
        } ;

        let split = s.split_once(" ") ;

        if let Some((prefix, suffix)) = split
        {
            match prefix.trim().to_ascii_lowercase().as_str()
            {
                "creature" =>

            }
        }



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
        // todo
        Ok(Classification::Enchantment)
    }
}

#[test]
fn classification_try_from_str__base()
{
    use crate::Classification::{*};
    assert_eq!(Classification::try_from("Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Ritual"), Ok(Ritual)) ;
    assert_eq!(Classification::try_from("Enchantment"), Ok(Enchantment)) ;
    assert_eq!(Classification::try_from("Legendary land"), Ok(LegendaryTerrain)) ;

    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Creature("Soldier"))) ;

}

#[test]
fn classification_try_from_str__case()
{
    assert_eq!(Classification::try_from("sorCerY"), Classification::try_from("Sorcery")) ;
    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Classification::Creature("  soldier "))) ;
}

/// test hors specification
#[test]
fn classification_try_from_str__whitespaces()
{
    assert_eq!(Classification::try_from(" Sorcery  "), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from(" Sorcery  "), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from(" Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Sorcery "), Ok(Sorcery)) ;

    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Classification::Creature("  soldier "))) ;
    assert_eq!(Classification::try_from("Creature  -   Soldier"), Ok(Classification::Creature("  soldier "))) ;
}

/// Ignores trailing whitespaces and case
/// ```
/// use mtg_multitool::Classification::* ;
/// assert_eq!(Sorcery, Sorcery) ;
/// assert_eq!(Ritual, Ritual) ;
/// assert_eq!(Enchantment, Enchantment) ;
/// assert_eq!(Terrain, Terrain) ;
/// assert_eq!(LegendaryTerrain, LegendaryTerrain) ;
///
/// assert_eq!(Creature("Soldier"), Creature("Soldier")) ;
/// assert_eq!(LegendaryCreature("Sauron"), LegendaryCreature("Sauron")) ;
///
/// assert_eq!(Artifact("???"), Artifact("???")) ;
///
/// assert_eq!(Token(Box::new(LegendaryCreature("Soldier"))), Token(Box::new(LegendaryCreature("Soldier")))) ;
/// ```
impl PartialEq for Classification<'_> {
    fn eq(&self, other: &Self) -> bool {
        use Classification::* ;
        match (self, other)
        {
            (Sorcery, Sorcery) |
            (Ritual, Ritual) |
            (Enchantment, Enchantment) |
            (Terrain, Terrain) |
            (LegendaryTerrain, LegendaryTerrain) => true,

            (Creature(self_str), Creature(other_str)) |
            (LegendaryCreature(self_str), LegendaryCreature(other_str)) |
            (Artifact(self_str), Artifact(other_str)) => self_str.trim().to_ascii_uppercase() == other_str.trim().to_ascii_uppercase(),

            (Token(self_inner), Token(other_inner)) => self_inner == other_inner,

            (_, _) => false
        }
    }
}

#[test]
fn classification_partialeq()
{
    use Classification::{*} ;

    assert_eq!(Sorcery, Sorcery) ;
    assert_eq!(Ritual, Ritual) ;
    assert_eq!(Enchantment, Enchantment) ;
    assert_eq!(Terrain, Terrain) ;
    assert_eq!(LegendaryTerrain, LegendaryTerrain) ;

    assert_eq!(Creature("Soldier"), Creature("Soldier")) ;
    assert_eq!(Creature("  soldieR "), Creature("Soldier")) ;
    assert_eq!(LegendaryCreature("Sauron"), LegendaryCreature("Sauron")) ;
    assert_eq!(LegendaryCreature("sAUron "), LegendaryCreature(" SauRON")) ;
    assert_ne!(LegendaryCreature("Sauron"), LegendaryCreature("Sauron, the necromancer")) ;
    assert_eq!(Artifact("???"), Artifact("???")) ;

    assert_eq!(Token(Box::new(LegendaryCreature("Soldier"))), Token(Box::new(LegendaryCreature("Soldier")))) ;
    assert_ne!(Token(Box::new(LegendaryCreature("Orc"))), Token(Box::new(LegendaryCreature("Soldier")))) ;
}

/// Represents a card's cost in the game's different manas<br/>
/// Example: `ManaCost(colorless, white, blue, black, red, green)`<br/>
/// ```
/// use mtg_multitool::ManaCost;
/// let mana = ManaCost::try_from("7 w bla black").unwrap() ;
///
/// assert_eq!(mana.colorless(), 7) ;
/// assert_eq!(mana.white(), 1) ;
/// assert_eq!(mana.black(), 2) ;
/// assert_eq!(mana.blue(), 0) ;
/// assert_eq!(mana.red(), 0) ;
/// assert_eq!(mana.green(), 0) ;
/// ```
#[derive(Debug, Clone,Copy)]
pub struct ManaCost(u8, u8, u8, u8, u8, u8) ;

impl ManaCost
{
    pub fn zero() -> ManaCost
    {
        ManaCost(0,0,0,0,0,0)
    }

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

impl TryFrom<&'_ str> for ManaCost
{
    type Error = &'static str;

    /// Converts str `n {"w" | "blu" | "bla" | "r" | "g"}` with n >= 0 to a ManaCost data struct
    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        if value.trim() == "" { return Err("&str is empty") ; }

        let mut result = ManaCost::zero() ;

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

#[cfg(LTR)]
include!("../assets/ltr/mod.rs");

