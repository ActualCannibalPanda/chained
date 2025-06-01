pub mod map;
pub mod player;

// #[macro_export]
// macro_rules! map_set {
//     ($tilemap_query:expr, $tile_query:expr, $w:expr, $h:expr, ($( $x:expr ),+) ) => {
//         for (tile_storage, _tile_size) in $tilemap_query.iter_mut() {
//             for x in 0..$w {
//                 for y in 0..$h {
//                     if x > 0 && x < $w - 1 && y > 0 && y < $h - 1 {
//                         if let Some(tile) = tile_storage.get(&TilePos { x, y }) {
//                             if let Ok(mut tile_texture) = $tile_query.get_mut(tile) {
//                                 tile_texture.0 = $($x);
//
//                             }
//                         }
//                     }
//                 }
//             }
//         }
