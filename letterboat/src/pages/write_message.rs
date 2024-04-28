use leptos::*;

#[component]
pub fn WriteMessage() -> impl IntoView {
    let message = create_rw_signal(String::new());

    let submit = move |_| {
        let _msg = message.get();
        // 여기에 메시지 제출 로직을 추가합니다.
        message.set(String::new());
    };

    view! {
        <h1>"Write a Message"</h1>
        <form on:submit=submit>
            <textarea
                value=message
                on:input=move |ev| message.set(event_target_value(&ev))
            />
            <button type="submit">"Send"</button>
        </form>
    }
}
