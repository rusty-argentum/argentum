use crate::entity::user::User;
use crate::entity::user::User::{Anonymous, Authenticated};
use crate::repository::session_repository::SessionRepositoryTrait;
use crate::repository::user_repository::{
    AnonymousUserRepositoryTrait, AuthenticatedUserRepositoryTrait,
};

pub struct UserAuthenticatesWithTokenUc<'a> {
    user_repository: &'a dyn AuthenticatedUserRepositoryTrait,
    anon_repository: &'a dyn AnonymousUserRepositoryTrait,
    session_repository: &'a dyn SessionRepositoryTrait,
}

impl<'a> UserAuthenticatesWithTokenUc<'a> {
    pub fn new(
        user_repository: &'a dyn AuthenticatedUserRepositoryTrait,
        anon_repository: &'a dyn AnonymousUserRepositoryTrait,
        session_repository: &'a dyn SessionRepositoryTrait,
    ) -> UserAuthenticatesWithTokenUc<'a> {
        UserAuthenticatesWithTokenUc {
            user_repository,
            anon_repository,
            session_repository,
        }
    }

    pub fn execute(&self, token: String) -> Result<User, AuthenticationError> {
        let session_result = self.session_repository.find_by_token(token);

        let session = match session_result {
            Some(s) => s,
            None => return Err(AuthenticationError::WrongToken),
        };

        let user = self.user_repository.find(&session.user_id);

        match user {
            Some(u) => Ok(Authenticated(u)),
            None => match self.anon_repository.find(&session.user_id) {
                Some(a) => Ok(Anonymous(a)),
                None => Err(AuthenticationError::UserNotFound),
            },
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("User not found")]
    UserNotFound,

    #[error("Wrong token")]
    WrongToken,
}

#[cfg(test)]
mod tests {
    use argentum_standard_business::data_type::email::EmailAddress;
    use argentum_standard_business::data_type::id::{IdTrait, IntId};

    use crate::entity::session::Session;
    use crate::entity::user::AuthenticatedUser;
    use crate::entity::user::User::{Anonymous, Authenticated};
    use crate::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
    use crate::mock::repository::authenticated_user_repository_mock::AuthenticatedUserRepositoryMock;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::repository::session_repository::SessionRepositoryTrait;
    use crate::repository::user_repository::AuthenticatedUserRepositoryTrait;
    use crate::use_case::user_authenticates_with_token::AuthenticationError;
    use crate::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
    use crate::value_object::name::Name;

    #[test]
    fn test_authenticates_with_token() -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousUserRepositoryMock::new();
        let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
        let session_repository = SessionRepositoryMock::new();

        //Data
        let user_id: Box<dyn IdTrait> = Box::new(IntId::new(1));
        let session_id = Box::new(IntId::new(2));
        let token = String::from("test-token");
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            Name::new(String::from("aaaa"), String::from("bbbb")).unwrap(),
            EmailAddress::new(String::from("aa@a.com")).unwrap(),
        );
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            &authenticated_user_repository,
            &anonymous_user_repository,
            &session_repository,
        );

        let result = uc.execute(token.clone());

        match result {
            Ok(u) => match u {
                Authenticated(u) => {
                    assert_eq!(u.id.to_string(), user_id.clone().to_string());

                    return Ok(());
                }
                Anonymous(_) => {
                    return Err("Return's anonymous user, not authenticated");
                }
            },
            Err(_) => {
                return Err("User is not authenticated");
            }
        }
    }

    #[test]
    fn test_authenticates_with_token_should_returns_error_if_token_invalid(
    ) -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousUserRepositoryMock::new();
        let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
        let session_repository = SessionRepositoryMock::new();

        //Data
        let user_id: Box<dyn IdTrait> = Box::new(IntId::new(1));
        let session_id = Box::new(IntId::new(2));
        let token = String::from("test-token");
        let authenticated_user = AuthenticatedUser::new(
            &user_id,
            Name::new(String::from("aaaa"), String::from("bbbb")).unwrap(),
            EmailAddress::new(String::from("aa@a.com")).unwrap(),
        );
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        authenticated_user_repository
            .save(&authenticated_user)
            .expect("Can't save a user");
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            &authenticated_user_repository,
            &anonymous_user_repository,
            &session_repository,
        );

        let result = uc.execute(String::from("wrong-test-token"));

        match result {
            Ok(_) => {
                return Err("Shpould return error, not an user");
            }
            Err(e) => match e {
                AuthenticationError::WrongToken => return Ok(()),
                _ => {
                    return Err("Invalid response status");
                }
            },
        }
    }

    #[test]
    fn test_authenticates_with_token_should_returns_error_if_user_doesnt_exist(
    ) -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousUserRepositoryMock::new();
        let authenticated_user_repository = AuthenticatedUserRepositoryMock::new();
        let session_repository = SessionRepositoryMock::new();

        //Data
        let user_id: Box<dyn IdTrait> = Box::new(IntId::new(1));
        let session_id = Box::new(IntId::new(2));
        let token = String::from("test-token");
        let session = Session::new(session_id, user_id.clone(), token.clone());

        //Prefilling
        session_repository
            .save(&session)
            .expect("Can't save a session");

        //Test
        let uc = UserAuthenticatesWithTokenUc::new(
            &authenticated_user_repository,
            &anonymous_user_repository,
            &session_repository,
        );

        let result = uc.execute(token.clone());

        match result {
            Ok(_) => {
                return Err("Shpould return error, not an user");
            }
            Err(e) => match e {
                AuthenticationError::UserNotFound => return Ok(()),
                _ => {
                    return Err("Invalid response status");
                }
            },
        }
    }
}