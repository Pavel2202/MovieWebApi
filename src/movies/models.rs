use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateMovie {
    pub title: String
}

#[derive(Deserialize, Clone)]
pub struct UpdateMovie {
    pub title: String
}