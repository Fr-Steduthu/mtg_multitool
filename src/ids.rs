use crate::Rarity;

#[derive(Debug, Clone, Copy)]
pub enum Id<'any>
{
    /// Example: `Name("Banish from Edoras")`
    Name(&'any str),

    /// Example: `Serial("LTR C 0001")`
    Serial(&'any str),

    /// Example: `Both("Banish from Edoras", "LTR C 0001")`
    Both(&'any str, &'any str),

    /// Example: `Either("Banish from Edoras", "LTR C 0001")`
    Either(&'any str, &'any str)
}

/// Used for the conversion, without moving the original value, of a type into an Id
pub trait AsId<'a>
{
    fn as_id(&self) -> Id<'a> ;
}

impl<'a, T> AsId<'a> for T where T: Into<Id<'a>> + Clone
{
    fn as_id(&self) -> Id<'a> {
        <T as Into<Id<'a>>>::into(self.clone())
    }
}

impl PartialEq for Id<'_>
{
    fn eq(&self, other: &Self) -> bool {
        use crate::ids::Id::* ;
        match (self, other)
        {
            (Both(n, id), Both(nn, idd)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase() && is_serial(id).unwrap() == is_serial(idd).unwrap(),
            (Either(n, id), Either(nn, idd)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase() || is_serial(id).unwrap() == is_serial(idd).unwrap(),

            (Name(n) | Either(n, _), Name(nn) | Either(nn, _)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase(),
            (Serial(id) | Either(_, id), Serial(idd) | Either(_, idd)) => is_serial(id).unwrap() == is_serial(idd).unwrap(),

            _ => false
        }
    }
}

#[test]
fn id_partialeq()
{
    use crate::ids::Id::{*} ;
    assert_eq!(Name("Banish from Edoras"), Both("BaNISH FROm EdORas", "LTR C 0001")) ;
    todo!()

}

/// Checks if given `&str` is formatted as a Serial would be ;<br/>
/// Returns `Some((series, rarity, id_within_series))` if it is, `None` otherwise
fn is_serial(s: &str) -> Option<(String, Rarity, usize)>
{
    let mut ss = s.split(" ") ;
    if let Some(series) = ss.next()
    {
        if let Some(rarity) = ss.next()
        {
            if let Ok(rarity) = crate::Rarity::try_from(rarity)
            {
                if let Some(id) = ss.next()
                {
                    if let Ok(id) = id.parse::<usize>()
                    {
                        return Some((series.trim().to_ascii_uppercase(), rarity, id)) ;
                    }
                }
            }
        }
    }

    None
}

#[test]
fn is_serial_tests()
{
    use crate::Rarity::* ;
    assert_eq!(is_serial("LTR"), None) ;
    assert_eq!(is_serial("LTR C"), None) ;
    assert_eq!(is_serial("LTR C 0001"), Some(("LTR".to_string(), Common, 1))) ;
    assert_eq!(is_serial("LTR C 1"), Some(("LTR".to_string(), Common, 1))) ;
    assert_eq!(is_serial("LTR c 0001"), Some(("LTR".to_string(), Common, 1))) ;
    assert_eq!(is_serial("C 001"), None) ;
    assert_eq!(is_serial("LTR 1"), None) ;
    assert_eq!(is_serial("ltr c 1"), Some(("LTR".to_string(), Common, 1))) ;
    assert_eq!(is_serial("c LTR 1"), None) ;
    assert_eq!(is_serial("1 c LTR"), None) ;
    assert_eq!(is_serial("Banish from Edoras"), None) ;
    assert_eq!(is_serial("LTR f 01"), None) ;
}

impl<'s> From<&'s str> for Id<'s>
{
    fn from(value: &'s str) -> Self {
        if is_serial(value).is_some()
        {
            Id::Serial(value)
        }
        else
        {
            Id::Name(value)
        }
    }
}

#[test]
fn id_from_str()
{
    todo!()
}
