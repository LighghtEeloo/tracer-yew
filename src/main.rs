#[allow(dead_code)]
#[allow(unused)]
pub mod cube;
pub mod model;
pub mod error;


fn main() {
    use model::Model;
    yew::start_app::<Model>();
}

pub mod prelude {
    pub use crate::LOG;
    pub use std::mem;

    pub use serde::{Deserialize, Serialize};
    pub use strum_macros::*;

    // pub use yew::web_sys::console::log_1 as LOG;

    #[macro_export]
    macro_rules! LOG {
        ($($arg:tt)*) => {{
            let res = format!($($arg)*);
            yew::web_sys::console::log_1(&res.into())
        }}
    }
}
