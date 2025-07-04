use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(CandidType, Serialize, Deserialize, Clone)]
struct UserProfile {
    username: String,
    bio: String,
}

type UserStorage = HashMap<Principal, UserProfile>;

thread_local! {
    static USERS: std::cell::RefCell<UserStorage> = std::cell::RefCell::new(HashMap::new());
}

#[ic_cdk::update]
fn set_user_profile(username: String, bio: String) {
    let caller = ic_cdk::api::caller();
    let profile = UserProfile { username, bio };

    USERS.with(|users| {
        users.borrow_mut().insert(caller, profile);
    });
}

#[ic_cdk::query]
fn get_user_profile() -> Option<UserProfile> {
    let caller = ic_cdk::api::caller();
    USERS.with(|users| users.borrow().get(&caller).cloned())
}

ic_cdk::export_candid!();
