pub mod chain;
pub mod cursor;
pub mod map;
pub mod player;
pub mod state;

#[macro_export]
macro_rules! map_string {
    ($($s:expr),+) => {
        {
            let mut s = String::new();
            $(
                s.push_str($s);
            )+
            s
        }
    };
}
