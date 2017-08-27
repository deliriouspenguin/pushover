mod user;
mod priority;
mod operating_system;
mod sound;

pub use self::user::User;
pub use self::priority::Priority;
pub use self::operating_system::OperatingSystem;
pub use self::sound::Sound;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum UserType {
    UserKey(String),
    Email(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct Message {
    pub id: u32,
    pub umid: u32,
    pub title: Option<String>,
    pub message: String,
    pub app: String,
    pub aid: u32,
    pub icon: String,
    pub date: u32,
    pub priority: Priority,
    pub sound: Option<String>,
    pub url: Option<String>,
    pub url_title: Option<String>,
    pub acked: i8,
    pub receipt: Option<String>,
    pub html: Option<i8>,
}
