mod mmap;
mod owned;

pub use crate::{
    mmap::MmapError,
    owned::{OwnedBuffer, OwnedIntoIter},
};

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        ops::Deref,
        path::{Path, PathBuf},
    };

    use super::*;

    #[test]
    fn owned_api() {
        fn static_assert<'a, T>()
        where
            T: TryFrom<&'a Path, Error = MmapError>,
            T: TryFrom<&'a PathBuf, Error = MmapError>,
            T: TryFrom<PathBuf, Error = MmapError>,
            T: TryFrom<&'a File, Error = MmapError>,
            T: TryFrom<File, Error = MmapError>,
            T: From<&'a Vec<u8>>,
            T: From<&'a [u8]>,
            &'a T: IntoIterator<Item = &'a u8> + 'a,
            T: Deref<Target = [u8]>,
            T: AsRef<[u8]>,
            T: bytes::Buf,
        {
        }

        static_assert::<OwnedBuffer>();
    }
}
