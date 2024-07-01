
const BANISH_FROM_EDORAS: &'static str = "LTR C 0001;Banish from Edoras;4 w;Sorcery;C;\"This spells costs 2 less to cast if it targets a tapped creature.\r\nExile target creature." ;

pub fn collection() -> Collection<'static>
{
    Collection::make(vec![BANISH_FROM_EDORAS])
}

#[test]
pub fn t()
{
    let _ = collection() ;
}