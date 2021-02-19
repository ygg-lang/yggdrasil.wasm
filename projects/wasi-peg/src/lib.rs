pub mod nodes;
pub mod states;
mod wit;

pub use crate::wit::PegHost;

wit_bindgen::generate!({


    world: "host",
});

export!(PegHost);
