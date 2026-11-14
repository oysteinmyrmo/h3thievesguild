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
        let end = self.pos.checked_add(len);
        match end.and_then(|end| self.data.get(self.pos..end).map(|slice| (end, slice))) {
            Some((end, slice)) => {
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
        let pos = self.pos;
        self.bytes(N)?.try_into().map_err(|_| OutOfBounds { pos, len: N })
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
    pub fn each<T: Copy + Default, const N: usize>(
        &mut self,
        mut read: impl FnMut(&mut Self) -> ReadResult<T>,
    ) -> ReadResult<[T; N]> {
        let mut out = [T::default(); N];
        for value in &mut out {
            *value = read(self)?;
        }
        Ok(out)
    }

    /// A string of `len` bytes. Bytes are taken as Latin-1, which is what the game uses.
    pub fn string(&mut self, len: usize) -> ReadResult<String> {
        Ok(latin1(self.bytes(len)?))
    }
}

/// Position of record `index` in an array of `size`-byte records starting at `start`.
pub(crate) fn record_pos(start: usize, index: usize, size: usize) -> ReadResult<usize> {
    index
        .checked_mul(size)
        .and_then(|offset| start.checked_add(offset))
        .ok_or(OutOfBounds { pos: start, len: size })
}

/// Decode Latin-1 bytes. Each byte maps to the Unicode code point of the same value.
pub(crate) fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}
