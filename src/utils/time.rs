use std::time::{SystemTime, UNIX_EPOCH};
pub fn get_unix_timestamp()->u32{
    let result=SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as u32;
    
    result
        
}
