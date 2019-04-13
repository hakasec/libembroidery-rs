use std::ffi::CString;

use crate::thread::EmbThreadList;
use crate::stitch::EmbStitchList;

pub struct EmbPattern {
    inner: *mut ffi::EmbPattern,
}

impl Drop for EmbPattern {
    fn drop(&mut self) {
        unsafe {
            ffi::embPattern_free(self.inner);
        }
    }
}

impl EmbPattern {
    pub fn new() -> EmbPattern {
        unsafe {
            EmbPattern {
                inner: ffi::embPattern_create(),
            }
        }
    }

    pub fn hide_stitches_over_length(&self, length: i32) {
        unsafe {
            ffi::embPattern_hideStitchesOverLength(self.inner, length);
        }
    }

    pub fn fix_colour_count(&self) {
        unsafe {
            ffi::embPattern_fixColorCount(self.inner);
        }
    }

    pub fn fix_color_count(&self) {
        self.fix_colour_count();
    }

    pub fn thread(&self) -> EmbThreadList {
        unsafe {
            EmbThreadList::from((*self.inner).threadList)
        }
    }

    pub fn scale(&self, scale: f64) {
        unsafe {
            ffi::embPattern_scale(self.inner, scale);
        }
    }

    pub fn stitches(&self) -> EmbStitchList {
        unsafe {
            EmbStitchList::from((*self.inner).stitchList)
        }
    }

    pub fn read(&self, filename: &str) -> Result<(), ()> {
        let s = CString::new(filename).unwrap();
        let ptr = s.as_bytes_with_nul().as_ptr();
        unsafe {
            match ffi::embPattern_read(self.inner, ptr as *const i8) {
                0 => Err(()),
                _ => Ok(()),
            }
        }
    }

    pub fn write(&self, filename: &str) -> Result<(), ()> {
        let s = CString::new(filename).unwrap();
        let ptr = s.as_bytes_with_nul().as_ptr();
        unsafe {
            match ffi::embPattern_write(self.inner, ptr as *const i8) {
                0 => Err(()),
                _ => Ok(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::drop;
    use crate::thread::EmbThread;

    #[test]
    fn test_create_drop() {
        let p = EmbPattern::new();
        drop(p);
    }

    #[test]
    fn test_file() {
        let p = EmbPattern::new();
        p.read("./examples/rose.pes").unwrap();

        assert_eq!(p.thread().len(), 1);
        assert_eq!(p.thread().get_at(0).description(), "Black");

        assert_eq!(p.stitches().len(), 5322);
        assert_eq!(p.stitches().get_at(0).flags(), 1);
    }
}
