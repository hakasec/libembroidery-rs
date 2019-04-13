extern crate libembroidery_sys as ffi;

mod pattern;
mod helpers;
mod colour;
mod thread;
// mod error;

pub use pattern::EmbPattern;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
