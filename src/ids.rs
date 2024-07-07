use crate::Rarity;

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
        match self {
            Id::Name(n) => if let Id::Name(nn) = other { n == nn } else { false },
            Id::Serial(s) => if let Id::Serial(ss) = other { s == ss } else { false },
            Id::Both(n, s) => {
                match other
                {
                    Id::Name(nn) => n == nn,
                    Id::Serial(ss) => s == ss,
                    Id::Both(nn, ss) => n == nn || s == ss,
                }
            }
        }
    }
}

#[test]
fn id_partialeq()
{
    todo!()
}

/// Checks if given `&str` is formatted as a Serial would be
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
