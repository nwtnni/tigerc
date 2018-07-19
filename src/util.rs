use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Void {}

impl fmt::Display for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}

#[macro_use]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = FnvHashMap::default();
            $(
                map.insert($key, $value);
            )*
            map
        }
    }
}
