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
