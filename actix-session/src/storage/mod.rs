//! Pluggable storage backends for session state.

#[cfg(feature = "cookie-session")]
mod cookie;
mod interface;
#[cfg(feature = "redis-session")]
mod redis_rs;
mod session_key;
#[cfg(feature = "redis-session")]
mod utils;

#[cfg(feature = "cookie-session")]
pub use self::cookie::CookieSessionStore;
pub use self::{
    interface::{LoadError, SaveError, SessionStore, UpdateError},
    session_key::SessionKey,
};
#[cfg(feature = "redis-session")]
pub use self::{
    redis_rs::{RedisSessionStore, RedisSessionStoreBuilder},
    utils::generate_session_key,
};
