//! I/O primitives.

use std::error::Error as StdError;

/// An I/O Error.
pub enum Error {
    /// General message
    Message(String)
}

/// Corresponds to std::io::Read
pub trait Read {
    /// Read some bytes
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    /// Read to end
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error>;

    /// React exact
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error>;
}

impl<T> Read for T where T: ::std::io::Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        <T as ::std::io::Read>::read(self, buf).map_err(|e| Error::Message(e.description().to_string()))
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        <T as ::std::io::Read>::read(self, buf).map_err(|e| Error::Message(e.description().to_string()))
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        <T as ::std::io::Read>::read_exact(self, buf).map_err(|e| Error::Message(e.description().to_string()))
    }
}
