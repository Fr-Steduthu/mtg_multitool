use crate::Rarity;

/// Examples: <br/>
/// `Serial("LTR C 0001")`
/// `Name("Banish from edoras")`
/// `Both("Banish from Edoras", "LTR C 0001")`
#[derive(Debug, Clone, Copy)]
pub enum Id<'s>
{
    Name(&'s str),
    Serial(&'s str),
    Both(&'s str, &'s str),
}

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
