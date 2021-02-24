use std::{fs, io, io::prelude::*, path::Path};

pub struct Driver<'a> {
    path: &'a Path
}

impl<'a> Driver<'a> {
    const BINARY: &'static [u8] = include_bytes!("../driver");

    pub fn try_new(path: &'a Path) -> io::Result<Self> {
        let this = Self { path };
        this.write_binary()?;
        this.prepare_executable()?;
        Ok(this)
    }

    fn prepare_executable(&self) -> io::Result<()> {
        // TODO: Check if exist
        if !self.exists_binary()? {
            self.write_binary()?;
        }
        if !self.is_executable()? {
            self.mark_executable()?;
        }
        Ok(())
    }

    fn exists_binary(&self) -> io::Result<bool> { Ok(false) }

    fn write_binary(&self) -> io::Result<()> {
        let mut file = fs::File::create(&self.path)?;
        file.write_all(Self::BINARY)?;
        Ok(())
    }

    fn is_executable(&self) -> io::Result<bool> { Ok(false) }

    fn mark_executable(&self) -> io::Result<()> {
        // TODO
        Ok(())
    }

    pub fn executable(&self) -> &Path { self.path }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let p: &Path = "target/driver".as_ref();
        let d = Driver::try_new(p).unwrap();
        println!("{:?}", d.executable());
        fs::remove_file(p).unwrap();
    }
}
