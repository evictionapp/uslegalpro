//! Common states with a method called "endpoint" used to construct the str for the endpoint

#[derive(Debug, Clone, Copy)]
pub enum State {
    Texas,
}

impl State {
    ///! get the endpoint as a &'static str
    pub fn endpoint(self) -> &'static str {
        match self {
            Self::Texas => "https://api-prod.uslegalpro.com/v2/tx",
        }
    }
}
