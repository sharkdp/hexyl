use std::convert::TryFrom;
use std::fs;
use std::io::{self, copy, sink, Read, Seek, SeekFrom};

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
        fn try_skip<R>(reader: R, pos: SeekFrom, err_desc: &'static str) -> io::Result<u64>
        where
            R: Read,
        {
            let cant_seek_abs_err = || Err(io::Error::new(io::ErrorKind::Other, err_desc));

            let offset = match pos {
                SeekFrom::Current(o) => u64::try_from(o).or_else(|_e| cant_seek_abs_err())?,
                SeekFrom::Start(_) | SeekFrom::End(_) => cant_seek_abs_err()?,
            };

            copy(&mut reader.take(offset), &mut sink())
        }

        match *self {
            Input::File(ref mut file) => {
                let seek_res = file.seek(pos);
                if let Err(Some(libc::ESPIPE)) = seek_res.as_ref().map_err(|err| err.raw_os_error())
                {
                    try_skip(
                        file,
                        pos,
                        "Pipes only support seeking forward with a relative offset",
                    )
                } else {
                    seek_res
                }
            }
            Input::Stdin(ref mut stdin) => try_skip(
                stdin,
                pos,
                "STDIN only supports seeking forward with a relative offset",
            ),
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
