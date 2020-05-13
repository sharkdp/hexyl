use std::fs;
use std::io::{self, Read, Seek, SeekFrom};

pub enum Input<'a> {
    File(fs::File),
    Stdin(io::StdinLock<'a>),
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::File(ref mut file) => file.read(buf),
            Input::Stdin(ref mut stdin) => stdin.read(buf),
        }
    }
}

impl<'a> Seek for Input<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match *self {
            Input::File(ref mut file) => {
                let seek_res = file.seek(pos);
                if let Err(Some(libc::ESPIPE)) = seek_res.as_ref().map_err(|err| err.raw_os_error())
                {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Using '--seek' is not supported when using a pipe",
                    )
                    .into());
                };
                seek_res
            }
            Input::Stdin(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Using '--seek' is not supported when reading from STDIN",
            )),
        }
    }
}

impl<'a> Input<'a> {
    pub fn into_inner(self) -> Box<dyn Read + 'a> {
        match self {
            Input::File(file) => Box::new(file),
            Input::Stdin(stdin) => Box::new(stdin),
        }
    }
}
