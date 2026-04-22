pub mod adapter;
pub mod message;
pub mod telegram;
pub mod discord;
pub mod cli;
pub mod webhook;

pub use adapter::ChannelAdapter;
pub use message::{PrimeMessage, Platform, Attachment};
