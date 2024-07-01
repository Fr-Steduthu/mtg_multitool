use std::convert::TryInto;
use crate::CsvConverter ;

pub struct LOTRMTGCard(&'static str) ;
impl TryInto<CsvConverter<'static>> for LOTRMTGCard
{
    type Error = <&'static str as TryInto<CsvConverter<'static>>>::Error;

    fn try_into(self) -> Result<CsvConverter<'static>, Self::Error>
    {
        self.0.try_into()
    }
}
