pub enum StitchType {
    Normal  = ffi::NORMAL as isize,
    Jump    = ffi::JUMP as isize,
    Trim    = ffi::TRIM as isize,
    Stop    = ffi::STOP as isize,
    Sequin  = ffi::SEQUIN as isize,
    End     = ffi::END as isize,
}

#[derive(Debug)]
pub struct EmbStitch {
    flags: i32,
    xx: f64,
    yy: f64,
    colour_idx: i32,
}

pub struct EmbStitchList {
    start: *mut ffi::EmbStitchList,
}

pub struct IntoIter {
    current: *mut ffi::EmbStitchList,
    count: i32,
    idx: i32,
}

impl From<ffi::EmbStitch> for EmbStitch {
    fn from(raw: ffi::EmbStitch) -> EmbStitch {
        EmbStitch::new(raw.flags, raw.xx, raw.yy, raw.color)
    }
}

impl Into<ffi::EmbStitch> for EmbStitch {
    fn into(self) -> ffi::EmbStitch {
        ffi::EmbStitch {
            flags: self.flags,
            xx: self.xx,
            yy: self.yy,
            color: self.colour_idx,
        }
    }
}

impl From<*mut ffi::EmbStitchList> for EmbStitchList {
    fn from(raw: *mut ffi::EmbStitchList) -> EmbStitchList {
        EmbStitchList {
            start: raw,
        }
    }
}

impl IntoIterator for EmbStitchList {
    type Item = EmbStitch;
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
    type Item = EmbStitch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.count {
            return None;
        }

        unsafe {
            let stitch = EmbStitch::from((*self.current).stitch);
            self.current = (*self.current).next;
            self.idx += 1;

            Some(stitch)
        }
    }
}

impl EmbStitch {
    pub fn new(flags: i32, xx: f64, yy: f64, idx: i32) -> EmbStitch {
        EmbStitch {
            flags,
            xx,
            yy,
            colour_idx: idx,
        }
    }

    pub fn flags(&self) -> i32 {
        self.flags
    }

    pub fn xx(&self) -> f64 {
        self.xx
    }

    pub fn yy(&self) -> f64 {
        self.yy
    }

    pub fn colour_idx(&self) -> i32 {
        self.colour_idx
    }
}

impl EmbStitchList {
    pub fn len(&self) -> i32 {
        unsafe {
            ffi::embStitchList_count(self.start)
        }
    }

    pub fn get_at(&self, idx: i32) -> EmbStitch {
        unsafe {
            EmbStitch::from(ffi::embStitchList_getAt(self.start, idx))
        }
    }
}
