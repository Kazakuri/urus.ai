use actix::Message;

use crate::db::implementation::Connection;
use crate::db::messages::user::ChangeUserPassword;
use crate::errors::UserError;
use crate::utils::{ verify_password, validate_and_hash_password };
use urusai_lib::models::user::User;

/// 
pub fn password_change(
    conn: &Connection,
    msg: &ChangeUserPassword,
) -> <ChangeUserPassword as Message>::Result {
    use diesel::dsl::count;
    use diesel::BelongingToDsl;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use urusai_lib::schema::users::dsl::*;

    if msg.current_password != msg.confirm_password {
        return Err(UserError::LoginError);
    }

    let user = users.filter(id.eq(&msg.id)).first::<User>(conn);
    
    match user {
        Ok(user) => {
            if verify_password(&user, &msg.current_password) {
                match validate_and_hash_password(msg.new_password.clone()) {
                    Ok(hash) => {
                        let user = diesel::update(&user)
                            .set(password_hash.eq(hash))
                            .get_result::<User>(conn);

                        match user {
                            Ok(user) => Ok(user),
                            Err(e) => Err(UserError::InternalError)
                        }
                    },
                    Err(e) => Err(e)
                }
            } else {
                Err(UserError::LoginError)
            }
        }
        Err(_) => Err(UserError::LoginError),
    }
}


// TODO: Test