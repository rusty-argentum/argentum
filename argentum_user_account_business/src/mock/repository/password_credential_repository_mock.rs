use crate::entity::credential::PasswordCredential;
use crate::repository::password_credential_repository::PasswordCredentialRepository;
use argentum_standard_business::data_type::id::Id;
use std::cell::RefCell;
use std::collections::HashMap;
/// TODO: NTS!!!!1111
pub struct PasswordCredentialRepositoryMock {
    credentials: RefCell<HashMap<String, PasswordCredential>>,
}

impl PasswordCredentialRepositoryMock {
    pub fn new() -> Self {
        PasswordCredentialRepositoryMock {
            credentials: RefCell::new(HashMap::new()),
        }
    }
}

impl Default for PasswordCredentialRepositoryMock {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordCredentialRepository for PasswordCredentialRepositoryMock {
    fn save(&self, cred: &PasswordCredential) {
        self.credentials
            .borrow_mut()
            .insert(cred.user_id.to_string(), cred.clone());
    }

    fn find_by_user_id(&self, id: &Id) -> Option<PasswordCredential> {
        self.credentials
            .borrow()
            .get(&*id.to_string())
            .map(|c| PasswordCredential::new(c.user_id.clone(), c.password.clone(), c.salt.clone()))
    }

    fn delete(&self, cred: &PasswordCredential) {
        self.credentials
            .borrow_mut()
            .remove(&cred.user_id.to_string());
    }
}
