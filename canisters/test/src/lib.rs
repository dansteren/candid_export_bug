use ic_cdk::api::call::{self, ManualReply};

#[derive(candid::CandidType)]
struct User {
    id: String,
}

#[ic_cdk_macros::query]
#[candid::candid_method(query)]
async fn fix() -> User {
    User {
        id: "a".to_string(),
    }
}

#[ic_cdk_macros::query(manual_reply = true)]
#[candid::candid_method(query)]
async fn method() -> ManualReply<User> {
    let user = User {
        id: "a".to_string(),
    };
    call::reply((user,));
    ManualReply::empty()
}

candid::export_service!();
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn write_candid_to_disk() {
        std::fs::write("test.did", export_candid()).unwrap();
    }
}
