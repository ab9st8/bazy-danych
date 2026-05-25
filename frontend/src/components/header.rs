use leptos::*;

#[component]
pub fn Header(
    user_id_input: RwSignal<String>,
    handle_save_user_id: impl Fn() + Clone + 'static,
    handle_new_user: impl Fn() + Clone + 'static,
) -> impl IntoView {
    view! {
        <div class="header">
            <div class="brand">
                <h1>"System Rezerwacji Biletów"</h1>
                <p>"Współbieżna i szybka rezerwacja biletów w czasie rzeczywistym z waitlistą"</p>
            </div>
            <div class="user-panel glass-card">
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <span style="font-size: 0.75rem; text-transform: uppercase; color: var(--text-muted); font-weight: 700;">
                        "Twój identyfikator użytkownika (User ID)"
                    </span>
                    <div style="display: flex; gap: 0.5rem;">
                        <input type="text" class="input-text"
                            prop:value=move || user_id_input.get()
                            on:input=move |ev| user_id_input.set(event_target_value(&ev))
                        />
                        <button class="btn" on:click=move |_| (handle_save_user_id)() >"Zapisz"</button>
                        <button class="btn btn-secondary" on:click=move |_| (handle_new_user)() title="Generuj losowy identyfikator">"Generuj nowy"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
