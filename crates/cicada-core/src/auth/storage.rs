use keyring::Entry;

use crate::auth::error::AuthError;
use crate::types::auth::AuthToken;

const SERVICE_NAME: &str = "com.cicada.app";

pub trait TokenStorage: Send + Sync {
    fn save_tokens(&self, tokens: &AuthToken) -> Result<(), AuthError>;
    fn load_tokens(&self) -> Result<Option<AuthToken>, AuthError>;
    fn clear_tokens(&self) -> Result<(), AuthError>;
}

pub struct KeyringStorage;

impl TokenStorage for KeyringStorage {
    fn save_tokens(&self, tokens: &AuthToken) -> Result<(), AuthError> {
        let entry = Entry::new(SERVICE_NAME, "cicada_auth")
            .map_err(|e| AuthError::ServerError(format!("keyring error: {}", e)))?;

        let json = serde_json::to_string(tokens)
            .map_err(|e| AuthError::InvalidResponse(format!("serialize error: {}", e)))?;

        entry
            .set_password(&json)
            .map_err(|e| AuthError::ServerError(format!("keyring set error: {}", e)))?;
        Ok(())
    }

    fn load_tokens(&self) -> Result<Option<AuthToken>, AuthError> {
        let entry = Entry::new(SERVICE_NAME, "cicada_auth")
            .map_err(|e| AuthError::ServerError(format!("keyring error: {}", e)))?;

        match entry.get_password() {
            Ok(json) => {
                let tokens: AuthToken = serde_json::from_str(&json)
                    .map_err(|e| AuthError::InvalidResponse(format!("deserialize error: {}", e)))?;
                Ok(Some(tokens))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(keyring::Error::Ambiguous(_)) => Ok(None),
            Err(e) => Err(AuthError::ServerError(format!("keyring get error: {}", e))),
        }
    }

    fn clear_tokens(&self) -> Result<(), AuthError> {
        let entry = Entry::new(SERVICE_NAME, "cicada_auth")
            .map_err(|e| AuthError::ServerError(format!("keyring error: {}", e)))?;

        entry
            .delete_credential()
            .map_err(|e| AuthError::ServerError(format!("keyring delete error: {}", e)))?;
        Ok(())
    }
}

pub struct MemoryStorage {
    tokens: std::sync::Mutex<Option<AuthToken>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            tokens: std::sync::Mutex::new(None),
        }
    }
}

impl TokenStorage for MemoryStorage {
    fn save_tokens(&self, tokens: &AuthToken) -> Result<(), AuthError> {
        let mut guard = self.tokens.lock().unwrap();
        *guard = Some(tokens.clone());
        Ok(())
    }

    fn load_tokens(&self) -> Result<Option<AuthToken>, AuthError> {
        let guard = self.tokens.lock().unwrap();
        Ok(guard.clone())
    }

    fn clear_tokens(&self) -> Result<(), AuthError> {
        let mut guard = self.tokens.lock().unwrap();
        *guard = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::auth::AuthToken;

    fn create_test_token() -> AuthToken {
        AuthToken {
            access_token: "test_access".to_string(),
            refresh_token: "test_refresh".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user_id: "test_user".to_string(),
        }
    }

    #[test]
    fn test_memory_save_and_load() {
        let storage = MemoryStorage::new();
        let token = create_test_token();

        storage.save_tokens(&token).unwrap();
        let loaded = storage.load_tokens().unwrap();

        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().access_token, "test_access");
    }

    #[test]
    fn test_memory_clear() {
        let storage = MemoryStorage::new();
        let token = create_test_token();

        storage.save_tokens(&token).unwrap();
        storage.clear_tokens().unwrap();

        let result = storage.load_tokens().unwrap();
        assert!(result.is_none());
    }
}
