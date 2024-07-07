
#[derive(Debug, Clone, Copy)]
pub enum Id<'s>
{
    /// Example: `Name("Banish from Edoras")`
    Name(&'s str),

    /// Example: `Serial("LTR C 0001")`
    Serial(&'s str),

    /// Example: `Both("Banish from Edoras", "LTR C 0001")`
    Both(&'s str, &'s str),
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
            (Name(n), Name(nn)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase(),
            (Serial(n), Serial(nn)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase(), //todo fix
            (Both(n, id), Both(nn, idd)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase() || id.to_ascii_lowercase() == idd.to_ascii_lowercase(),

            (Both(n, _), Name(nn)) |
            (Both(_, n), Serial(nn)) |

            (Name(n), Both(nn, _)) |
            (Serial(n), Both(_, nn)) => n.to_ascii_lowercase() == nn.to_ascii_lowercase(),

            _ => false

            /*Id::Name(n) => if let Id::Name(nn) = other { n.to_ascii_lowercase() == nn.to_ascii_lowercase() } else { false },
            Id::Serial(s) => if let Id::Serial(ss) = other { s.to_ascii_lowercase() == ss.to_ascii_lowercase() } else { false },
            Id::Both(n, s) => {
                match other
                {
                    Id::Name(nn) => n.to_ascii_lowercase() == nn.to_ascii_lowercase(),
                    Id::Serial(ss) => s.to_ascii_lowercase() == ss.to_ascii_lowercase(),
                    Id::Both(nn, ss) => n.to_ascii_lowercase() == nn.to_ascii_lowercase() || s.to_ascii_lowercase() == ss.to_ascii_lowercase(),
                }
            }*/
        }
    }
}

#[test]
fn id_partialeq()
{
    use crate::ids::Id::{*} ;
    assert_eq!(Name("Banish from Edoras"), Both("BaNISH FROm EdORas", "LTR C 0001")) ;

}

/// Checks if given `&str` is formatted as a Serial would be
fn is_serial(s: &str) -> bool
{
    let mut ss = s.split(" ") ;
    if let Some(_) = ss.next()
    {
        if let Some(rarity) = ss.next()
        {
            if let Ok(_) = crate::Rarity::try_from(rarity)
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
        if is_serial(value)
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
