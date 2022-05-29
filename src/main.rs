use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]

//define struct Todo - https://jsonplaceholder.typicode.com/todos?userId=1
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

//TOKIO
//serialize - deserialize
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Receive type-checked JSON
    //vector client request
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;
    //result console
    println!("{:#?}", todos);

    // Send and receive type-checked JSON
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "Test message Rust".to_owned(),
        completed: false
    };
    //create new_todo request
    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    // Send and receive arbitrary JSON
    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        //json macro - Serde
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Test Send Json Macro".to_owned(),
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);
    Ok(())
}
