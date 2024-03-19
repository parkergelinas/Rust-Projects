use actix_web::{web, HttpResponse};
use std::sync::Mutex;
use uuid::Uuid;
use crate::models::{AppState, User};

// Create a new user
pub async fn create_user(item: web::Json<User>, data: web::Data<AppState>) -> HttpResponse {
    let mut users = data.users.lock().unwrap();
    let user = User {
        id: Uuid::new_v4(),
        username: item.username.clone(),
        email: item.email.clone(),
    };
    users.push(user);
    HttpResponse::Created().json("User created")
}

// Retrieve a user by ID
pub async fn get_user(id: web::Path<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let users = data.users.lock().unwrap();
    let user = users.iter().find(|user| user.id == *id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json("User not found"),
    }
}

// Update a user by ID
pub async fn update_user(id: web::Path<Uuid>, item: web::Json<User>, data: web::Data<AppState>) -> HttpResponse {
    let mut users = data.users.lock().unwrap();
    let idx = users.iter().position(|user| user.id == *id);
    match idx {
        Some(idx) => {
            let user = &mut users[idx];
            user.username = item.username.clone();
            user.email = item.email.clone();
            HttpResponse::Ok().json("User updated")
        },
        None => HttpResponse::NotFound().json("User not found"),
    }
}

// Delete a user by ID
pub async fn delete_user(id: web::Path<Uuid>, data: web::Data<AppState>) -> HttpResponse {
    let mut users = data.users.lock().unwrap();
    let idx = users.iter().position(|user| user.id == *id);
    match idx {
        Some(idx) => {
            users.remove(idx);
            HttpResponse::Ok().json("User deleted")
        },
        None => HttpResponse::NotFound().json("User not found"),
    }
}