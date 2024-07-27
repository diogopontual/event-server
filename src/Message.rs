use std::io::Cursor;
use anyhow::Result;
pub struct Message{

}

impl Message{
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<()> {
        Ok(())
    }
}