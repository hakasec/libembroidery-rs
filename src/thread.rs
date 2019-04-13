use std::ffi::CStr;

use crate::colour::{EmbColour, EmbColor};

#[derive(Debug)]
pub struct EmbThread {
    colour: EmbColour,
    description: String,
    catalog_number: String,
}

pub struct EmbThreadList {
    start: *mut ffi::EmbThreadList,
}

pub struct IntoIter {
    current: *mut ffi::EmbThreadList,
    count: i32,
    idx: i32,
}

impl From<ffi::EmbThread> for EmbThread {
    fn from(raw: ffi::EmbThread) -> EmbThread {
        unsafe {
            let desc = CStr::from_ptr(raw.description).to_str().unwrap();
            let catn = CStr::from_ptr(raw.catalogNumber).to_str().unwrap();
            EmbThread::new(
                EmbColour::from(raw.color),
                desc,
                catn,
            )
        }
    }
}

impl From<*mut ffi::EmbThreadList> for EmbThreadList {
    fn from(raw: *mut ffi::EmbThreadList) -> Self {
        EmbThreadList {
            start: raw,
        }
    }
}

impl IntoIterator for EmbThreadList {
    type Item = EmbThread;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: self.start,
            count: self.len(),
            idx: 0,
        }
    }
}

impl Iterator for IntoIter {
    type Item = EmbThread;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.count {
            return None;
        }

        unsafe {
            let thread = EmbThread::from((*self.current).thread);
            self.current = (*self.current).next;
            self.idx += 1;

            Some(thread)
        }
    }
}

impl EmbThread {
    pub fn new(colour: EmbColour, desc: &str, catn: &str) -> EmbThread {
        EmbThread {
            colour,
            description: String::from(desc),
            catalog_number: String::from(catn),
        }
    }

    pub fn colour(self) -> EmbColour {
        self.colour
    }

    pub fn color(self) -> EmbColor {
        self.colour()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn catalog_number(&self) -> &str {
        self.catalog_number.as_str()
    }
}

impl EmbThreadList {
    pub fn len(&self) -> i32 {
        unsafe {
            ffi::embThreadList_count(self.start)
        }
    }

    pub fn get_at(&self, idx: i32) -> EmbThread {
        unsafe {
            EmbThread::from(ffi::embThreadList_getAt(self.start, idx))
        }
    }
}

