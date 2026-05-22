// src/components/root.rs
// This component is not actively used; the main app view is in app.rs.
// Kept for reference.

use leptos::*;

#[component]
pub fn Root() -> impl IntoView {
    view! {
        // Header
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
                        <input type="text" class="input-text" prop:value="" />
                        <button class="btn">"Zapisz"</button>
                        <button class="btn btn-secondary" title="Generuj losowy identyfikator">"Generuj nowy"</button>
                    </div>
                </div>
            </div>
        </div>

        // Dashboard layout
        <div class="dashboard-grid">
            <div class="event-list">
                <h2 style="font-size: 1.25rem; margin-bottom: 0.5rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted);">
                    "Dostępne wydarzenia"
                </h2>
            </div>

            <div class="detail-panel">
            </div>
        </div>

        // Toasts
        <div class="toast-container">
        </div>
    }
}
