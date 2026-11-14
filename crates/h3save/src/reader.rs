//! Little-endian reads from the decompressed save data.

/// A cursor over a byte slice. Reads past the end return an error instead of panicking.
pub(crate) struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

/// A read went past the end of the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutOfBounds {
    pub pos: usize,
    pub len: usize,
}

pub(crate) type ReadResult<T> = Result<T, OutOfBounds>;

impl<'a> Reader<'a> {
    pub fn at(data: &'a [u8], pos: usize) -> Self {
        Reader { data, pos }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn bytes(&mut self, len: usize) -> ReadResult<&'a [u8]> {
        let end = self.pos.checked_add(len).filter(|&end| end <= self.data.len());
        match end {
            Some(end) => {
                let slice = &self.data[self.pos..end];
                self.pos = end;
                Ok(slice)
            }
            None => Err(OutOfBounds { pos: self.pos, len }),
        }
    }

    pub fn skip(&mut self, len: usize) -> ReadResult<()> {
        self.bytes(len).map(|_| ())
    }

    pub fn array<const N: usize>(&mut self) -> ReadResult<[u8; N]> {
        let mut out = [0u8; N];
        out.copy_from_slice(self.bytes(N)?);
        Ok(out)
    }

    pub fn u8(&mut self) -> ReadResult<u8> {
        Ok(self.array::<1>()?[0])
    }

    pub fn u16(&mut self) -> ReadResult<u16> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    pub fn u32(&mut self) -> ReadResult<u32> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    /// `N` consecutive values read with `read`.
    pub fn each<T, const N: usize>(&mut self, mut read: impl FnMut(&mut Self) -> ReadResult<T>) -> ReadResult<[T; N]> {
        let mut out = Vec::with_capacity(N);
        for _ in 0..N {
            out.push(read(self)?);
        }
        Ok(out
            .try_into()
            .unwrap_or_else(|_| unreachable!("exactly N values were pushed")))
    }

    /// A string of `len` bytes. Bytes are taken as Latin-1, which is what the game uses.
    pub fn string(&mut self, len: usize) -> ReadResult<String> {
        Ok(latin1(self.bytes(len)?))
    }
}

/// Decode Latin-1 bytes. Each byte maps to the Unicode code point of the same value.
pub(crate) fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}

/// Index of the first occurrence of `needle` in `haystack`, like `std::search` in C++.
pub(crate) fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack.windows(needle.len()).position(|window| window == needle)
}
