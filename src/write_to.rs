use std::io::Write;
use std::io::Result;

pub trait WriteTo<W: Write> {
    fn write_to(&self, w: &mut W) -> Result<()>;
}
