use crate::models::user::User;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Trait que define o contrato do repositório
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Option<User>;
}

// Implementação em memória (pra começar simples)
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<u64, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        
        // Alguns usuários de exemplo
        users.insert(1, User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });
        users.insert(2, User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });
        
        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: u64) -> Option<User> {
        let users = self.users.read().await;
        users.get(&id).cloned()
    }
}