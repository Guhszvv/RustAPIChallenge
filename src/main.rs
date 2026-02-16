mod handlers;
mod models;
mod repositories;
mod services;

use axum::{
    routing::get,
    Router,
};
use repositories::user_repository::InMemoryUserRepository;
use services::user_service::UserService;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 1. Cria o repository
    let repository = Arc::new(InMemoryUserRepository::new());
    
    // 2. Injeta o repository no service
    let service = UserService::new(repository);
    
    // 3. Cria as rotas passando o service como State
    let app = Router::new()
        .route("/users/{id}", get(handlers::user_handler::get_user))
        .with_state(service);
    
    // 4. Inicia o servidor
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor rodando em http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
