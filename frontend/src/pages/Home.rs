use leptos::*;
use crate::components::Navbar;
use crate::components::Footer;
use crate::api::todo_api;
use crate::stores::todo_store::use_todo_store;

#[component]
pub fn Home() -> impl IntoView {
    let todo_store = use_todo_store();
    let (new_todo, set_new_todo) = create_signal("".to_string());

    let add_todo = move |ev: web_sys::Event| {
        ev.prevent_default();
        if !new_todo.get().is_empty() {
            todo_store.add_todo(new_todo.get());
            set_new_todo("".to_string());
        }
    };

    view! {
        <div class="min-h-screen flex flex-col">
            <Navbar />
            <div class="flex-grow container mx-auto px-4 py-8">
                <div class="max-w-2xl mx-auto">
                    <h1 class="text-3xl font-bold text-gray-800 mb-8">Todo List</h1>
                    
                    <form class="flex gap-4 mb-8" on:submit=add_todo>
                        <input
                            type="text"
                            class="flex-1 px-4 py-2 border rounded-lg focus:outline-none focus:border-blue-500"
                            placeholder="Add a new todo..."
                            prop:value=new_todo
                            on:input=move |ev| set_new_todo(event_target_value(&ev))
                        />
                        <button
                            type="submit"
                            class="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
                        >
                            Add
                        </button>
                    </form>

                    <div class="space-y-4">
                        {move || todo_store.todos.get().into_iter().map(|todo| {
                            view! {
                                <div class="flex items-center justify-between p-4 bg-white rounded-lg shadow">
                                    <div class="flex items-center gap-4">
                                        <input
                                            type="checkbox"
                                            class="w-5 h-5"
                                            prop:checked=todo.completed
                                            on:change=move |_| todo_store.toggle_todo(todo.id)
                                        />
                                        <span class={move || if todo.completed { "line-through text-gray-500" } else { "text-gray-800" }}>
                                            {todo.title}
                                        </span>
                                    </div>
                                    <button
                                        class="text-red-500 hover:text-red-700"
                                        on:click=move |_| todo_store.delete_todo(todo.id)
                                    >
                                        Delete
                                    </button>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
            <Footer />
        </div>
    }
}