use leptos::*;
use leptos_router::*;

#[derive(Clone)]
struct MessageData {
    message: String,
    nickname: String,
}

#[component]
pub fn WriteMessage() -> impl IntoView {
    let message_data = create_rw_signal(MessageData {
        message: String::new(),
        nickname: String::new(),
    });

    let submit = move |_| {
        let data = message_data.get();
        let _msg = data.message.clone();
        let _nickname = data.nickname.clone();
        // 여기에 메시지 제출 로직을 추가합니다.
        message_data.set(MessageData {
            message: String::new(),
            nickname: String::new(),
        });
    };

    view! {
        <div class="flex flex-col items-center justify-center min-h-screen">
            <header class="flex items-center justify-between w-full max-w-4xl py-4 px-6 bg-white shadow-md">
                <A href="/" class="text-xl font-bold text-gray-800">"LetterBoat"</A>
            </header>
            <main class="flex flex-col items-center justify-center w-full max-w-4xl">
                <h1 class="text-3xl font-bold mb-8">"Write a boat to send"</h1>
                <form class="w-full max-w-2xl" on:submit=submit>
                    <div class="flex flex-col space-y-4">
                        <textarea
                            class="w-full h-48 p-2 border border-gray-300 rounded bg-white text-gray-700 text-base"
                            placeholder="Type your message..."
                            prop:value=message_data.get().message
                            on:input=move |ev| message_data.update(|data| data.message = event_target_value(&ev))
                        />
                        <input
                            type="text"
                            class="w-full p-2 border border-gray-300 rounded bg-white text-gray-700 text-base"
                            placeholder="Enter your nickname"
                            prop:value=message_data.get().nickname
                            on:input=move |ev| message_data.update(|data| data.nickname = event_target_value(&ev))
                        />
                        <button
                            class="bg-teal-500 hover:bg-teal-700 text-white font-bold py-2 px-4 rounded"
                            type="submit"
                        >
                            "Send"
                        </button>
                    </div>
                </form>
                <div class="mt-8">
                                    <p class="text-xl">"Message: " {message_data.get().message}</p>
                                    <p class="text-xl">"Nickname: " {message_data.get().nickname}</p>
                                </div>
            </main>
        </div>
    }
}
