use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[component]
pub fn App() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::new());
    let (new_todo, set_new_todo) = create_signal(String::new());

    let fetch_todos = create_action(move |_| async move {
        let response = reqwest::get("http://localhost:8000/api/todos").await.unwrap();
        let todos: Vec<Todo> = response.json().await.unwrap();
        set_todos.update(|t| *t = todos);
    });

    let add_todo = create_action(move |(title,): &(String,)| {
        let title = title.to_string();
        async move {
            let client = reqwest::Client::new();
            let response = client
                .post("http://localhost:8000/api/todos")
                .json(&serde_json::json!({ "title": title }))
                .send()
                .await
                .unwrap();
            let todo: Todo = response.json().await.unwrap();
            set_todos.update(|t| t.push(todo));
            set_new_todo.set(String::new());
        }
    });

    let toggle_todo = create_action(move |(id, completed): &(i32, bool)| {
        let (id, completed) = (*id, *completed);
        async move {
            let client = reqwest::Client::new();
            let response = client
                .put(&format!("http://localhost:8000/api/todos/{}", id))
                .json(&serde_json::json!({ "completed": completed }))
                .send()
                .await
                .unwrap();
            let updated_todo: Todo = response.json().await.unwrap();
            set_todos.update(|t| {
                if let Some(todo) = t.iter_mut().find(|t| t.id == id) {
                    *todo = updated_todo;
                }
            });
        }
    });

    let delete_todo = create_action(move |id: &i32| {
        let id = *id;
        async move {
            let client = reqwest::Client::new();
            client
                .delete(&format!("http://localhost:8000/api/todos/{}", id))
                .send()
                .await
                .unwrap();
            set_todos.update(|t| t.retain(|todo| todo.id != id));
        }
    });

    create_effect(move |_| {
        fetch_todos.dispatch(());
    });

    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold mb-8 text-center">Todo App</h1>
            
            <div class="mb-8">
                <form class="flex gap-4" on:submit=move |ev| {
                    ev.prevent_default();
                    add_todo.dispatch((new_todo.get(),));
                }>
                    <input
                        type="text"
                        class="flex-1 px-4 py-2 border rounded focus:outline-none focus:border-blue-500"
                        placeholder="Add a new todo..."
                        prop:value=move || new_todo.get()
                        on:input=move |ev| {
                            set_new_todo.set(event_target_value(&ev));
                        }
                    />
                    <button
                        type="submit"
                        class="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none"
                    >
                        "Add"
                    </button>
                </form>
            </div>

            <ul class="space-y-4">
                {move || todos.get().into_iter().map(|todo| {
                    view! {
                        <li class="flex items-center justify-between p-4 bg-white rounded shadow">
                            <div class="flex items-center gap-4">
                                <input
                                    type="checkbox"
                                    class="w-5 h-5"
                                    prop:checked=todo.completed
                                    on:change=move |ev| {
                                        toggle_todo.dispatch((todo.id, event_target_checked(&ev)));
                                    }
                                />
                                <span class=move || {
                                    if todo.completed {
                                        "line-through text-gray-500"
                                    } else {
                                        "text-gray-900"
                                    }
                                }>
                                    {todo.title}
                                </span>
                            </div>
                            <button
                                class="px-3 py-1 text-red-500 hover:text-red-700 focus:outline-none"
                                on:click=move |_| delete_todo.dispatch(todo.id)
                            >
                                "Delete"
                            </button>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> });
}
