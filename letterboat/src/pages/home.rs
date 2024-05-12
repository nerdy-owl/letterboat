use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-screen">
            <h1 class="text-4xl font-bold mb-4">"Welcome to Leptos!"</h1>
            <p class="text-xl mb-8">"This is the home page."</p>
>            <a href="/write" class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded">
                "Go to Write Message"
            </a>
          <hr/> 
            <a href="/letterview" class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded">
                "Go to letter viewe"
            </a>



        </div>
    }
}
