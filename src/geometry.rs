
use serde::Deserialize;

mod geometry {
    #[derive(Debug, Clone, Deserialize)]
    pub struct Point {
        pub x: f64,
        pub y: f64
    }
}