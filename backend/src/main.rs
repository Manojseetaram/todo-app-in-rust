mod db;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use db::connect_to_db;
use futures_util::stream::StreamExt;
use models::{CreateTodoItem, TodoItem, UpdateTodoItem};
use mongodb::{
    bson::{doc, Binary, spec::BinarySubtype},
    Collection,
};
use uuid::Uuid;


struct AppState {
    todo_collection: Collection<TodoItem>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let filter = doc! {};
    let mut cursor = data.todo_collection.find(filter).await.unwrap();

    let mut todos: Vec<TodoItem> = Vec::new();
    while let Some(result) = cursor.next().await {
        if let Ok(todo) = result {
            todos.push(todo);
        }
    }
    HttpResponse::Ok().json(todos)
}

async fn add_todo(
    item: web::Json<CreateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let new_todo = TodoItem {
        id: Binary {
            subtype: BinarySubtype::Uuid,
            bytes: Uuid::new_v4().as_bytes().to_vec(),
        },
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };

    match data.todo_collection.insert_one(new_todo.clone()).await {
        Ok(_) => HttpResponse::Ok().json(new_todo),
        Err(_) => HttpResponse::InternalServerError().body("Error adding todo"),
    }
}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let uuid = path.into_inner();
    let filter = doc! {
        "id": Binary {
            subtype: BinarySubtype::Uuid,
            bytes: uuid.as_bytes().to_vec()
        }
    };

    let mut update_doc = doc! {};
    if let Some(title) = &item.title {
        update_doc.insert("title", title.clone());
    }
    if let Some(completed) = item.completed {
        update_doc.insert("completed", completed);
    }

    let update = doc! { "$set": update_doc };

    match data.todo_collection.update_one(filter, update).await {
        Ok(r) if r.matched_count > 0 => HttpResponse::Ok().body("Todo updated"),
        Ok(_) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating todo"),
    }
}

async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let uuid = path.into_inner();
    println!("🔹 DELETE called for UUID: {}", uuid);

    let filter = doc! {
        "id": Binary {
            subtype: BinarySubtype::Uuid,
            bytes: uuid.as_bytes().to_vec()
        }
    };
    println!("MongoDB filter: {:?}", filter);

    match data.todo_collection.delete_one(filter).await {
        Ok(r) if r.deleted_count > 0 => {
            println!("Successfully deleted");
            HttpResponse::Ok().body("Todo deleted")
        },
        Ok(_) => {
            println!("No matching todo found");
            HttpResponse::NotFound().body("Todo not found")
        },
        Err(e) => {
            println!("Delete error: {:?}", e);
            HttpResponse::InternalServerError().body("Delete failed")
        }
    }
}



async fn delete_all_todos(data: web::Data<AppState>) -> impl Responder {
    let result = data
        .todo_collection
        .delete_many(doc! {})
        .await;

    match result {
        Ok(delete_result) => {
            println!("Deleted {} todos", delete_result.deleted_count);
            HttpResponse::Ok().body(format!("Deleted {} todos", delete_result.deleted_count))
        }
        Err(err) => {
            println!("Failed to delete all todos: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to delete all todos")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = connect_to_db().await;
    let todo_collection = db.collection::<TodoItem>("todos");

    let app_state = web::Data::new(AppState { todo_collection });

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos",web::delete().to(delete_all_todos) )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}