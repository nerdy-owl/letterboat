use leptos::*;

#[component]
pub fn LetterView() -> impl IntoView {
    view! {
        <div class="max-w-2xl mx-auto p-6 bg-white shadow-md rounded-lg">
            <h1 class="text-2xl font-bold mb-4">"메시지 제목"</h1>
            <div class="border-t border-gray-200 mt-4">
                메시지 컨텐트
            </div>
        </div>
    }
}
