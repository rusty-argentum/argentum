use crate::entity::session::Session;
use crate::repository::session_repository::{SessionRepositoryError, SessionRepositoryTrait};
use argentum_standard_business::data_type::id::{Id, IdFactory};
use argentum_user_business::entity::user::{AnonymousUser, UserTrait};
use argentum_user_business::repository::user_repository::{
    AnonymousUserRepositoryTrait, SavingUserError,
};
use argentum_user_business::token::GeneratorTrait;

pub struct AnonymousRegistersUc<'s> {
    id_factory: &'s dyn IdFactory,
    user_repository: &'s dyn AnonymousUserRepositoryTrait,
    session_repository: &'s dyn SessionRepositoryTrait,
    token_generator: &'s dyn GeneratorTrait,
}

impl<'s> AnonymousRegistersUc<'s> {
    pub fn new(
        id_factory: &'s dyn IdFactory,
        user_repository: &'s dyn AnonymousUserRepositoryTrait,
        session_repository: &'s dyn SessionRepositoryTrait,
        token_generator: &'s dyn GeneratorTrait,
    ) -> AnonymousRegistersUc<'s> {
        AnonymousRegistersUc {
            id_factory,
            user_repository,
            session_repository,
            token_generator,
        }
    }

    pub fn execute(&self, id: &Id) -> Result<(AnonymousUser, Session), AnonymousRegistrationError> {
        let user = {
            let user = AnonymousUser::new(&id);

            let result = self.user_repository.save(&user);

            match result {
                Ok(_) => user,
                Err(e) => return Err(AnonymousRegistrationError::SavingAnonymousError(e)),
            }
        };

        let session = Session::new(
            self.id_factory.create(),
            user.id().clone(),
            self.token_generator.generate(&id),
        );

        match self.session_repository.save(&session) {
            Ok(_) => Ok((user, session)),
            Err(e) => Err(AnonymousRegistrationError::SavingSessionError(e)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AnonymousRegistrationError {
    #[error("Can't save anonymous")]
    SavingAnonymousError(#[from] SavingUserError),

    #[error("Can't save session")]
    SavingSessionError(#[from] SessionRepositoryError),
}

#[cfg(test)]
mod tests {
    use crate::mock::repository::broken::session_repository_mock::SessionRepositoryMockWithBrokenSave;
    use crate::mock::repository::session_repository_mock::SessionRepositoryMock;
    use crate::mock::token::TokenGeneratorMock;
    use crate::use_case::anonymous_registers::{AnonymousRegistersUc, AnonymousRegistrationError};
    use argentum_standard_business::data_type::id::{Id, IdFactory};
    use argentum_standard_business::mock::data_type::id_factory::IdFactoryMock;
    use argentum_user_business::mock::repository::anonymous_user_repository_mock::AnonymousUserRepositoryMock;
    use argentum_user_business::mock::repository::broken::anonymous_user_repository_mock::AnonymousRepositoryMockWithBrokenSave;

    #[test]
    fn anonymous_registers() -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousUserRepositoryMock::new();
        let session_repository = SessionRepositoryMock::new();
        let id_factory = IdFactoryMock::new();
        let token_generator = TokenGeneratorMock::new();

        let uc = AnonymousRegistersUc::new(
            &id_factory,
            &anonymous_user_repository,
            &session_repository,
            &token_generator,
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok((anonymous, s)) => {
                assert_eq!(anonymous.id.to_string(), anon_id.clone().to_string());
                assert_eq!(s.user_id.to_string(), anon_id.clone().to_string());

                return Ok(());
            }
            Err(_) => {
                return Err("User is not registered");
            }
        }
    }

    #[test]
    fn anonymous_registers_with_broken_user_repository() -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousRepositoryMockWithBrokenSave::new();
        let session_repository = SessionRepositoryMock::new();
        let id_factory = IdFactoryMock::new();
        let token_generator = TokenGeneratorMock::new();

        let uc = AnonymousRegistersUc::new(
            &id_factory,
            &anonymous_user_repository,
            &session_repository,
            &token_generator,
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                AnonymousRegistrationError::SavingAnonymousError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
    #[test]
    fn anonymous_registers_with_broken_session_repository() -> Result<(), &'static str> {
        let anonymous_user_repository = AnonymousUserRepositoryMock::new();
        let session_repository = SessionRepositoryMockWithBrokenSave::new();
        let id_factory = IdFactoryMock::new();
        let token_generator = TokenGeneratorMock::new();

        let uc = AnonymousRegistersUc::new(
            &id_factory,
            &anonymous_user_repository,
            &session_repository,
            &token_generator,
        );

        let anon_id: Id = id_factory.create();
        let result = uc.execute(&anon_id);

        match result {
            Ok(_) => Err("Should return error"),
            Err(e) => match e {
                AnonymousRegistrationError::SavingSessionError(_) => Ok(()),
                _ => Err("Wrong error type"),
            },
        }
    }
}
