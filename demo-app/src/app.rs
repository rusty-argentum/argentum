use argentum_log_business::LoggerTrait;
use argentum_standard_business::data_type::email::EmailAddress;
use argentum_standard_business::data_type::id::IdFactory;
use argentum_user_account_business::use_case::anonymous_registers::AnonymousRegistersUc;
use argentum_user_account_business::use_case::user_authenticates_with_token::UserAuthenticatesWithTokenUc;
use argentum_user_account_business::use_case::user_logins_with_password::UserLoginsWithPasswordUc;
use argentum_user_account_business::use_case::user_registers_with_password::UserRegistersWithPasswordUc;
use argentum_user_business::entity::user::AnonymousUser;
use argentum_user_business::value_object::name::Name;

pub struct App<'s> {
    id_factory: &'s dyn IdFactory,
    anonymous_registers_uc: &'s AnonymousRegistersUc<'s>,
    user_logins_with_pw: &'s UserLoginsWithPasswordUc<'s>,
    user_registers_with_pw: &'s UserRegistersWithPasswordUc<'s>,
    user_authenticates_with_token: &'s UserAuthenticatesWithTokenUc<'s>,
    logger: &'s dyn LoggerTrait,
}

impl<'s> App<'s> {
    pub fn new(
        id_factory: &'s dyn IdFactory,
        anonymous_registers_uc: &'s AnonymousRegistersUc<'s>,
        user_logins_with_pw: &'s UserLoginsWithPasswordUc<'s>,
        user_registers_with_pw: &'s UserRegistersWithPasswordUc<'s>,
        user_authenticates_with_token: &'s UserAuthenticatesWithTokenUc<'s>,
        logger: &'s dyn LoggerTrait,
    ) -> App<'s> {
        App {
            id_factory,
            anonymous_registers_uc,
            user_logins_with_pw,
            user_registers_with_pw,
            user_authenticates_with_token,
            logger,
        }
    }

    pub fn run(&self) -> Result<(), String> {
        self.logger.trace("Demo trace log".to_string());
        self.logger.debug("Demo debug log".to_string());
        self.logger.info("Demo info log".to_string());
        self.logger.warning("Demo warning log".to_string());
        self.logger.error("Demo error log".to_string());
        self.logger.critical("Demo critical log".to_string());

        let anon_id = self.id_factory.create();

        let anon_registration_result = self.anonymous_registers_uc.execute(&anon_id);
        let (_, anon_session) = match anon_registration_result {
            Ok(r) => r,
            Err(e) => return Err(e.to_string()),
        };

        let anon_auth_result = self
            .user_authenticates_with_token
            .execute(anon_session.token);

        match anon_auth_result {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e.to_string());
                return Err(e.to_string());
            }
        };

        let user_id = self.id_factory.create();

        let name_res = Name::new(String::from("Sarah"), String::from("Connor"));
        let name = match name_res {
            Ok(name) => name,
            Err(e) => return Err(e.to_string()),
        };
        let email_res = EmailAddress::new(String::from("sarah-connor@example.com"));
        let email = match email_res {
            Ok(email) => email,
            Err(e) => return Err(e.to_string()),
        };
        let password = String::from("111");

        let res = self
            .user_registers_with_pw
            .execute(user_id, name, email, password);
        match res {
            Ok(_) => {
                println!("Registered")
            }
            Err(e) => {
                println!("Error: {}", e)
            }
        }

        let anon_id2 = self.id_factory.create();
        let anon2 = AnonymousUser::new(&anon_id2);
        let email_res2 = EmailAddress::new(String::from("sarah-connor@example.com"));
        let password2 = String::from("111");

        let email2 = match email_res2 {
            Ok(email) => email,
            Err(e) => return Err(e.to_string()),
        };

        let login_result = self.user_logins_with_pw.execute(anon2, email2, password2);

        let login = match login_result {
            Ok(l) => {
                println!("Logged In");
                l
            }
            Err(e) => {
                println!("Login error: {}", e.to_string());
                return Err(e.to_string());
            }
        };

        let aut_result = self.user_authenticates_with_token.execute(login.token);

        match aut_result {
            Ok(_) => {
                println!("Authenticated");
                Ok(())
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                Err(e.to_string())
            }
        }
    }
}
