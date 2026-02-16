use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
    
    pub async fn get_user_by_id(&self, id: u64) -> Result<User, String> {
        // Aqui você pode adicionar validações, regras de negócio, etc
        if id == 0 {
            return Err("ID inválido".to_string());
        }
        
        self.repository
            .find_by_id(id)
            .await
            .ok_or_else(|| format!("Usuário com ID {} não encontrado", id))
    }
}