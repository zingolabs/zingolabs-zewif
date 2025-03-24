use anyhow::{Result, bail};

use super::super::Data;

pub trait Parse {
    fn parse(p: &mut Parser) -> Result<Self>
    where
        Self: Sized;

    fn parse_buf(buf: &dyn AsRef<[u8]>, trace: bool) -> Result<Self>
    where
        Self: Sized,
    {
        let mut p = Parser::new(&buf);
        p.set_trace(trace);
        let result = Self::parse(&mut p)?;
        p.check_finished()?;
        Ok(result)
    }
}

pub trait ParseWithParam<P> {
    fn parse(p: &mut Parser, param: P) -> Result<Self>
    where
        Self: Sized;

    #[allow(dead_code)]
    fn parse_buf(buf: &dyn AsRef<[u8]>, param: P, trace: bool) -> Result<Self>
    where
        Self: Sized,
    {
        let mut p = Parser::new(&buf);
        p.set_trace(trace);
        let result = Self::parse(&mut p, param)?;
        p.check_finished()?;
        Ok(result)
    }
}

pub struct Parser<'a> {
    pub buffer: &'a [u8],
    pub offset: usize,
    pub trace: bool,
}

impl std::fmt::Debug for Parser<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
            .field("offset", &self.offset)
            .field("len", &self.len())
            .field("remaining", &self.remaining())
            .finish()
    }
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a dyn AsRef<[u8]>) -> Self {
        Self { buffer: buffer.as_ref(), offset: 0, trace: false }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn remaining(&self) -> usize {
        self.len() - self.offset
    }

    pub fn check_finished(&self) -> Result<()> {
        if self.offset < self.buffer.len() {
            bail!("Buffer has {} bytes left", self.remaining());
        }
        Ok(())
    }

    pub fn next(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.offset + n > self.buffer.len() {
            bail!(
                "Buffer underflow at offset {}, needed {} bytes, only {} remaining",
                self.offset,
                n,
                self.remaining()
            );
        }
        let bytes = &self.buffer[self.offset..self.offset + n];
        self.offset += n;
        if self.trace {
            println!(
                "\t🟢 next({}): {:?} remaining: {} peek: {:?}",
                n,
                hex::encode(bytes),
                self.remaining(),
                hex::encode(self.peek(100))
            );
        }
        Ok(bytes)
    }

    pub fn peek(&self, n: usize) -> &'a [u8] {
        let available = std::cmp::min(n, self.remaining());
        &self.buffer[self.offset..self.offset + available]
    }

    pub fn rest(&mut self) -> Data {
        Data::parse_len(self, self.remaining()).unwrap()
    }

    pub fn peek_rest(&self) -> Data {
        Data::from_slice(&self.buffer[self.offset..])
    }

    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }

    pub fn trace(&self, msg: &str) {
        if self.trace {
            println!("🔵 {}: {:?}", msg, self.peek_rest());
        }
    }
}

impl std::io::Read for &mut Parser<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let parser = &mut **self;
        let n = std::cmp::min(buf.len(), parser.remaining());
        buf[..n].copy_from_slice(&parser.buffer[parser.offset..parser.offset + n]);
        parser.offset += n;
        Ok(n)
    }
}
