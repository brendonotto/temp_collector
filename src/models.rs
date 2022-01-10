mod models {
    use chrono::{DateTime, Local};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Reading {
        pub room : String,
        pub temperature : f64,
        pub humidity: f64,
        pub timestamp : DateTime<Local>
    }
}