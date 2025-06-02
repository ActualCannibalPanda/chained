pub mod map;
pub mod player;

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
