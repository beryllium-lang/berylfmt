use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrSlice<'a> {
    start: &'a u8,
    pub len: usize,
}

impl StrSlice<'_> {
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.to_str_slice()).into_owned()
    }

    pub fn from_str(str: &str) -> Self {
        Self {
            start: unsafe { &*str.as_ptr() },
            len: str.len(),
        }
    }

    pub fn to_str_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.start, self.len)
        }
    }

    pub unsafe fn from_raw_parts(start: *const u8, len: usize) -> Self {
        Self {
            start: &*start,
            len: len,
        }
    }
}

impl<'a> Index<usize> for StrSlice<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            let start_ptr = self.start as *const u8;
            &*(start_ptr.add(index))
        }
    }
}
