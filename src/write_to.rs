use std::io::Write;
use std::io::Result;

pub trait WriteTo {
    fn write_to<W: Write>(&self, w: &mut W) -> Result<()>;
}
