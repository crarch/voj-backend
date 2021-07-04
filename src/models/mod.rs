pub mod user;
pub mod user_id;

pub use user_id::UserId;
pub use user::User;

use serde::{Deserialize,Serialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct Pass{
    pass:Vec<i32>,
}