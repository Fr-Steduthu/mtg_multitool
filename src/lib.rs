

/// Represents card unique identifiers (either name or `{series} {rarity} {internal series id}`)
pub mod ids;

/// Represents a generic card
pub mod cards;

/// Represents cards collections
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
pub enum Classification
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
    Creature(String),

    /// `"Token [...]"`
    Token(Box<Classification>),
    /// `"Legendary [...]"`
    Legendary(Box<Classification>),
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
impl<'mostly_static> TryFrom<&'mostly_static str> for Classification
{
    type Error = &'static str;

    fn try_from(s: &'mostly_static str) -> Result<Self, Self::Error>
    {
        use Classification::{*} ;
        let s = s.trim().to_ascii_lowercase() ;
        let s = match s.as_str()
        {
            "sorcery" => return Ok(Sorcery),
            "ritual" => return Ok(Ritual),
            "enchantment" => return Ok(Enchantment),
            "land" | "terrain" | "basic land" => return Ok(Terrain),
            "artifact" => return Ok(Artifact),

            other => other,
        } ;


        let split = s.split_once(" ") ;
        if let Some((prefix, suffix)) = split
        {
            return match prefix.trim().to_ascii_lowercase().as_str()
            {
                "token" => Ok(Token(Box::new(Classification::try_from(suffix)?))),
                "legendary" => Ok(Legendary(Box::new(Classification::try_from(suffix)?))),

                "creature" => {
                    let new_split = s.split_once("-");
                    if let Some((label, class)) = new_split
                    {
                        return if label.trim() == "creature"
                        {
                            Ok(Creature(class.trim().to_string()))
                        } else {
                            Err("Expected a creature")
                        }
                    }

                    Err("Could not parse str into card kind")
                },

                _ => Err("Could not recognize card kind")
            }
        }

        Err("Could not recognize card kind")
    }
}

#[test]
fn classification_try_from_str_base()
{
    use crate::Classification::{*};
    assert_eq!(Classification::try_from("Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Ritual"), Ok(Ritual)) ;
    assert_eq!(Classification::try_from("Enchantment"), Ok(Enchantment)) ;
    assert_eq!(Classification::try_from("Legendary land"), Ok(Legendary(Box::new(Terrain)))) ;

    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Creature("Soldier".to_string()))) ;

}

#[test]
fn classification_try_from_str_case()
{
    assert_eq!(Classification::try_from("sorCerY"), Classification::try_from("Sorcery")) ;
    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Classification::Creature("  soldier ".to_string()))) ;
}

/// Test hors specification
#[test]
fn classification_try_from_str_whitespaces()
{
    use Classification::{Sorcery, Creature} ;
    assert_eq!(Classification::try_from(" Sorcery  "), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from(" Sorcery  "), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from(" Sorcery"), Ok(Sorcery)) ;
    assert_eq!(Classification::try_from("Sorcery "), Ok(Sorcery)) ;

    assert_eq!(Classification::try_from("Creature - Soldier"), Ok(Creature("  soldier ".to_string()))) ;
    assert_eq!(Classification::try_from("Creature  -   Soldier"), Ok(Creature("  soldier ".to_string()))) ;
}

/// Ignores trailing whitespaces and case
/// ```
/// use mtg_multitool::Classification::* ;
/// assert_eq!(Sorcery, Sorcery) ;
/// assert_eq!(Ritual, Ritual) ;
/// assert_eq!(Enchantment, Enchantment) ;
/// assert_eq!(Terrain, Terrain) ;
/// //assert_eq!(LegendaryTerrain, LegendaryTerrain) ;
///
/// assert_eq!(Creature("Soldier".to_string()), Creature("Soldier".to_string())) ;
/// //assert_eq!(LegendaryCreature("Sauron".to_string()), LegendaryCreature("Sauron".to_string())) ;
///
/// assert_eq!(Artifact, Artifact) ;
///
/// //assert_eq!(Token(Box::new(LegendaryCreature("Soldier".to_string()))), Token(Box::new(LegendaryCreature("Soldier".to_string())))) ;
/// ```
impl PartialEq for Classification {
    fn eq(&self, other: &Self) -> bool {
        use Classification::* ;
        match (self, other)
        {
            (Sorcery, Sorcery) |
            (Ritual, Ritual) |
            (Enchantment, Enchantment) |
            (Terrain, Terrain) |
            (Artifact, Artifact) => true,

            (Creature(self_str), Creature(other_str)) => self_str.trim().to_ascii_uppercase() == other_str.trim().to_ascii_uppercase(),

            (Legendary(self_inner), Legendary(other_inner)) |
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
    assert_eq!(Legendary(Box::new(Terrain)), Legendary(Box::new(Terrain))) ;
    assert_eq!(Artifact, Artifact) ;

    assert_eq!(Creature("Soldier".to_string()), Creature("Soldier".to_string())) ;
    assert_eq!(Creature("  soldieR ".to_string()), Creature("Soldier".to_string())) ;
    assert_eq!(Legendary(Box::new(Creature("Sauron".to_string()))), Legendary(Box::new(Creature("Sauron".to_string())))) ;
    assert_eq!(Legendary(Box::new(Creature("SaURon ".to_string()))), Legendary(Box::new(Creature(" SaurON".to_string())))) ;
    assert_ne!(Legendary(Box::new(Creature("Sauron".to_string()))), Legendary(Box::new(Creature("Sauron, the Necromancer".to_string())))) ;


    assert_eq!(Token(Box::new(Creature("Soldier".to_string()))), Token(Box::new(Creature("Soldier".to_string())))) ;
    assert_ne!(Token(Box::new(Creature("Orc".to_string()))), Token(Box::new(Creature("Soldier".to_string())))) ;
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
    /// Creates a ManaCost object with all values set to 0
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

/// Converts str `n {"w" | "blu" | "bla" | "r" | "g"}` with n >= 0 to a ManaCost data struct
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
                    return Err("No colorless cost given, please specify '0' if this is intentional")
                }
            }
        ;

        use std::str::FromStr;

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

#[test]
fn ltr_collection()
{
    let lua = ltr::collection() ;

    lua.add(&BANISH_FROM_EDORAS) ;

    println!("{:?}", lua)
}

