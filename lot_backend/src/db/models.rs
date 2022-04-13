use chrono::NaiveDateTime;
use crate::db::schema::tbl_user;

#[allow(non_snake_case)]
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone, Insertable, Default)]
#[table_name = "tbl_user"]
pub struct User {
    pub uuid : i64,
    pub userID : Option<String>,
    pub userPW : Option<String>,
    pub nickname : Option<String>,
    pub exceptArena : Option<i32>,
    pub regLastLoginDate : Option<NaiveDateTime>,
    pub regDate : Option<NaiveDateTime>,
    pub regIP : Option<String>,
    pub walletAddress : Option<String>,
    pub verifyEmailHash : Option<String>,
    pub verifyEmail : Option<u8>,
    pub txHash : Option<String>,
    pub profileImage : Option<String>,
}

impl User {
    pub fn new_email_hash(verify_email_hash : String) -> User{
        User {
            verifyEmailHash : Some(verify_email_hash),
            ..Default::default()
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct VerifyUser {
    pub email : String,
    pub wallet_address : String,
}

#[derive(Serialize, Deserialize)]
pub struct InsertableUser {
    pub wallet_address : String,
    pub txhash : String,
    pub nickname : String,
    pub profile_image : String,
}