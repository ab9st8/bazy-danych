use crate::api::{self, Event, ReserveResult, Seat};
use crate::components::Header;
use leptos::*;
use std::collections::HashSet;
use uuid::Uuid;

// ---------- Toasts ----------

#[derive(Clone, Debug)]
pub struct Toast {
    pub id: Uuid,
    pub message: String,
    pub toast_type: ToastType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
}

// ---------- LocalStorage helpers ----------

fn get_stored_user_id() -> Uuid {
    let local_storage = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten());

    if let Some(storage) = local_storage {
        if let Some(val) = storage.get_item("user_id").ok().flatten() {
            if let Ok(id) = Uuid::parse_str(&val) {
                return id;
            }
        }
        let new_id = Uuid::new_v4();
        let _ = storage.set_item("user_id", &new_id.to_string());
        new_id
    } else {
        Uuid::new_v4()
    }
}

fn get_stored_waitlists() -> HashSet<Uuid> {
    let local_storage = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten());

    if let Some(storage) = local_storage {
        if let Some(val) = storage.get_item("waitlisted_events").ok().flatten() {
            if let Ok(set) = serde_json::from_str::<HashSet<Uuid>>(&val) {
                return set;
            }
        }
    }
    HashSet::new()
}

fn save_waitlists(set: &HashSet<Uuid>) {
    let local_storage = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten());
    if let Some(storage) = local_storage {
        if let Ok(val) = serde_json::to_string(set) {
            let _ = storage.set_item("waitlisted_events", &val);
        }
    }
}

// ---------- Root component ----------

#[component]
pub fn App() -> impl IntoView {
    // Identity & Global State
    let user_id = create_rw_signal(get_stored_user_id());
    let user_id_input = create_rw_signal(user_id.get_untracked().to_string());
    let waitlisted_events = create_rw_signal(get_stored_waitlists());
    let toasts = create_rw_signal(Vec::<Toast>::new());

    // Active Selection State
    let events = create_rw_signal(Vec::<Event>::new());
    let selected_event_id = create_rw_signal(None::<Uuid>);
    let seats = create_rw_signal(Vec::<Seat>::new());

    // Loading indicator signals
    let is_reserving = create_rw_signal(false);
    let is_paying = create_rw_signal(false);

    // Timer for active hold
    let remaining_seconds = create_rw_signal(0.0);

    // Sync input box when user_id updates
    {
        let user_id = user_id;
        let user_id_input = user_id_input;
        create_effect(move |_| {
            user_id_input.set(user_id.get().to_string());
        });
    }

    // Save User ID to local storage when changed
    {
        let user_id = user_id;
        create_effect(move |_| {
            let id = user_id.get();
            let local_storage = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten());
            if let Some(storage) = local_storage {
                let _ = storage.set_item("user_id", &id.to_string());
            }
        });
    }

    // Save Waitlist to local storage when changed
    {
        let waitlisted_events = waitlisted_events;
        create_effect(move |_| {
            save_waitlists(&waitlisted_events.get());
        });
    }

    // Helper: Add Toast notification
    let add_toast = {
        let toasts = toasts;
        move |msg: String, t: ToastType| {
            let id = Uuid::new_v4();
            toasts.update(|v| v.push(Toast { id, message: msg, toast_type: t }));
            set_timeout(
                move || {
                    toasts.update(|v| v.retain(|toast| toast.id != id));
                },
                std::time::Duration::from_secs(4),
            );
        }
    };

    // Polling triggers
    let fetch_events_trigger = create_rw_signal((0u32));
    let fetch_seats_trigger = create_rw_signal((0u32));

    // Event list resource loader
    let events_resource = create_resource(
        move || fetch_events_trigger.get(),
        |_| async move { api::fetch_events().await },
    );

    // Update local events state when resources finish
    {
        let events = events;
        let selected_event_id = selected_event_id;
        create_effect(move |_| {
            if let Some(Ok(evs)) = events_resource.get() {
                events.set(evs.clone());
                if selected_event_id.get_untracked().is_none() && !evs.is_empty() {
                    selected_event_id.set(Some(evs[0].id));
                }
            }
        });
    }

    // Seat map resource loader
    let seats_resource = create_resource(
        move || (selected_event_id.get(), fetch_seats_trigger.get()),
        |(evt_id, _)| async move {
            if let Some(id) = evt_id {
                api::fetch_seats(id).await
            } else {
                Ok(Vec::new())
            }
        },
    );

    // Update local seats state
    {
        let seats = seats;
        create_effect(move |_| {
            if let Some(Ok(sts)) = seats_resource.get() {
                seats.set(sts);
            }
        });
    }

    // Background polling – setup once on first render
    {
        let fetch_events_trigger = fetch_events_trigger;
        let fetch_seats_trigger = fetch_seats_trigger;
        create_effect(move |prev: Option<()>| {
            if prev.is_some() {
                return;
            }
            let _ = set_interval_with_handle(
                move || {
                    fetch_events_trigger.update(|n| *n += 1);
                },
                std::time::Duration::from_secs(3),
            );
            let _ = set_interval_with_handle(
                move || {
                    fetch_seats_trigger.update(|n| *n += 1);
                },
                std::time::Duration::from_secs(2),
            );
        });
    }

    // Derived signal: Active hold for the current user
    let active_hold = create_memo(move |_| {
        let current_uid = user_id.get();
        seats.with(|sts| {
            sts.iter()
                .find(|s| s.status == "held" && s.held_by_user_id == Some(current_uid))
                .cloned()
        })
    });

    // Automatically transition off waitlist if user has been assigned a hold
    {
        let add_toast = add_toast.clone();
        create_effect(move |_| {
            if active_hold.get().is_some() {
                if let Some(evt_id) = selected_event_id.get() {
                    let is_on_wl = waitlisted_events.get().contains(&evt_id);
                    if is_on_wl {
                        waitlisted_events.update(|set| {
                            set.remove(&evt_id);
                        });
                        add_toast(
                            "Zwolniono miejsce z waitlisty! Miejsce zostało dla Ciebie zarezerwowane!"
                                .to_string(),
                            ToastType::Success,
                        );
                    }
                }
            }
        });
    }

    // Selected event memo
    let selected_event = create_memo(move |_| {
        let sel_id = selected_event_id.get()?;
        events.get().into_iter().find(|e| e.id == sel_id)
    });

    // Countdown Timer logic for the hold
    {
        let remaining_seconds = remaining_seconds;
        create_effect(move |_| {
            if let Some(hold) = active_hold.get() {
                if let Some(until) = hold.held_until_epoch {
                    let remaining_seconds = remaining_seconds;
                    let update_timer = move || {
                        let now_epoch = js_sys::Date::now() / 1000.0;
                        let rem = until - now_epoch;
                        remaining_seconds.set(rem.max(0.0));
                    };
                    update_timer();
                    if let Ok(handle) = set_interval_with_handle(
                        update_timer,
                        std::time::Duration::from_millis(200),
                    ) {
                        on_cleanup(move || handle.clear());
                    }
                }
            } else {
                remaining_seconds.set(0.0);
            }
        });
    }

    // Formatted time representation MM:SS
    let formatted_time = move || {
        let secs = remaining_seconds.get().round() as i32;
        if secs <= 0 {
            "00:00".to_string()
        } else {
            let m = secs / 60;
            let s = secs % 60;
            format!("{:02}:{:02}", m, s)
        }
    };

    // Action: Trigger seat reservation / waitlist
    let handle_reserve = {
        let add_toast = add_toast.clone();

        move || {
            let evt_id = match selected_event_id.get() {
                Some(id) => id,
                None => return,
            };
            let uid = user_id.get();

            is_reserving.set(true);
            let add_toast = add_toast.clone();
            spawn_local(async move {
                match api::reserve_seat(evt_id, uid).await {
                    Ok(ReserveResult::Reserved(_res)) => {
                        add_toast(
                            "Zarezerwowano miejsce! Rozpoczęto 30s na opłacenie.".to_string(),
                            ToastType::Success,
                        );
                        fetch_seats_trigger.update(|n| *n += 1);
                        fetch_events_trigger.update(|n| *n += 1);
                    }
                    Ok(ReserveResult::Waitlisted(_)) => {
                        waitlisted_events.update(|set| {
                            set.insert(evt_id);
                        });
                        add_toast(
                            "Brak wolnych biletów. Zostałeś dodany do waitlisty (kolejki)!"
                                .to_string(),
                            ToastType::Warning,
                        );
                        fetch_seats_trigger.update(|n| *n += 1);
                        fetch_events_trigger.update(|n| *n += 1);
                    }
                    Err(err) => {
                        add_toast(format!("Rezerwacja nieudana: {}", err), ToastType::Error);
                    }
                }
                is_reserving.set(false);
            });
        }
    };

    // Action: Pay for the current reservation
    let handle_pay = {
        let add_toast = add_toast.clone();

        move |res_id: Uuid| {
            let uid = user_id.get();
            is_paying.set(true);
            let add_toast = add_toast.clone();
            spawn_local(async move {
                match api::pay_reservation(res_id, uid).await {
                    Ok(_) => {
                        add_toast(
                            "Płatność udana! Zakupiono bilet.".to_string(),
                            ToastType::Success,
                        );
                        fetch_seats_trigger.set((0u32));
                        fetch_events_trigger.set((0u32));
                    }
                    Err(err) => {
                        add_toast(format!("Płatność nieudana: {}", err), ToastType::Error);
                    }
                }
                is_paying.set(false);
            });
        }
    };

    // Action: Refresh Identity (Generate new random user ID)
    let handle_new_user = {
        let add_toast = add_toast.clone();

        move || {
            let new_uid = Uuid::new_v4();
            user_id.set(new_uid);
            add_toast(
                "Zmieniono tożsamość. Wygenerowano nowy User ID.".to_string(),
                ToastType::Success,
            );
            fetch_seats_trigger.set((0u32));
        }
    };

    // Action: Manually apply customized User ID from input box
    let handle_save_user_id = {
        let add_toast = add_toast.clone();

        move || {
            match Uuid::parse_str(&user_id_input.get()) {
                Ok(parsed) => {
                    user_id.set(parsed);
                    add_toast("Zapisano User ID.".to_string(), ToastType::Success);
                    fetch_seats_trigger.set((0u32));
                }
                Err(_) => {
                    add_toast("Niepoprawny format UUID!".to_string(), ToastType::Error);
                }
            }
        }
    };

    // Clone closures for use in callbacks
    let handle_reserve_for_click = handle_reserve.clone();
    let handle_pay_for_click = handle_pay.clone();
    let add_toast_for_waitlist = add_toast.clone();

    // ---------- VIEW ----------

    view! {
        // Header component
        <Header
            user_id_input=user_id_input
            handle_save_user_id=handle_save_user_id
            handle_new_user=handle_new_user
        />

        // Main content
        <div class="dashboard-grid">
            // Event list sidebar
            <div class="event-list">
                <h2 style="font-size: 1.25rem; margin-bottom: 0.5rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted);">
                    "Dostępne wydarzenia"
                </h2>
                <For
                    each=move || events.get()
                    key=|e| e.id
                    children=move |evt: Event| {
                        let evt_id = evt.id;
                        let is_selected = move || selected_event_id.get() == Some(evt_id);
                        view! {
                            <div
                                class=move || if is_selected() { "event-card event-card.active" } else { "event-card" }
                                on:click=move |_| selected_event_id.set(Some(evt_id))
                            >
                                <h3>{evt.name.clone()}</h3>
                                <p class="event-venue">{evt.venue.clone()}</p>
                                <p class="event-date">{evt.date.clone()}</p>
                                <div class="event-stats">
                                    <span class="stat stat-available">"Wolne: " {evt.available_seats}</span>
                                    <span class="stat stat-held">"Hold: " {evt.held_seats}</span>
                                    <span class="stat stat-sold">"Sprzedane: " {evt.sold_seats}</span>
                                    {if evt.waitlist_count > 0 {
                                        view! { <span class="stat stat-waitlist">"Kolejka: " {evt.waitlist_count}</span> }.into_view()
                                    } else {
                                        ().into_view()
                                    }}
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            // Right panel: detail view
            <div class="detail-panel">

                // Status Banner Zone
                {move || if let Some(hold) = active_hold.get() {
                    let res_id = hold.reservation_id;
                    let handle_pay = handle_pay_for_click.clone();
                    view! {
                        <div class="hold-banner">
                            <div class="hold-time-wrapper">
                                <span class="timer-icon">"⏰"</span>
                                <div>
                                    <h4 style="color: var(--accent-info); font-size: 1.1rem; margin-bottom: 0.2rem;">
                                        "ZAREZERWOWANO MIEJSCE!"
                                    </h4>
                                    <p style="font-size: 0.8rem; color: var(--text-main); opacity: 0.9;">
                                        "Masz zarezerwowane miejsce o etykiecie "
                                        <strong style="color: #ffffff; font-size: 0.95rem;">
                                            "#" {hold.label.clone().unwrap_or_else(|| "?".to_string())}
                                        </strong>
                                        ". Opłać bilet przed upływem czasu."
                                    </p>
                                </div>
                            </div>
                            <div style="display: flex; align-items: center; gap: 1rem; flex-wrap: wrap;">
                                <div class="hold-time">{formatted_time()}</div>
                                {move || if let Some(r_id) = res_id {
                                    let handle_pay = handle_pay.clone();
                                    view! {
                                        <button
                                            class="btn btn-success"
                                            disabled=move || is_paying.get()
                                            on:click=move |_| handle_pay(r_id)
                                        >
                                            {move || if is_paying.get() {
                                                view! { <div class="spinner"></div> }.into_view()
                                            } else {
                                                "Opłać bilet".into_view()
                                            }}
                                        </button>
                                    }.into_view()
                                } else {
                                    view! { <span style="font-size: 0.75rem; color: var(--text-muted);">"Brak ID rezerwacji"</span> }.into_view()
                                }}
                            </div>
                        </div>
                    }.into_view()
                } else if let Some(_evt) = selected_event.get() {
                    let evt_id2 = _evt.id;
                    let is_wl = move || waitlisted_events.get().contains(&evt_id2);
                    if is_wl() {
                        let add_toast = add_toast_for_waitlist.clone();
                        view! {
                            <div class="waitlist-banner">
                                <div style="display: flex; gap: 0.75rem; align-items: center;">
                                    <span style="font-size: 1.5rem;">"⏳"</span>
                                    <div>
                                        <h4 style="color: var(--accent-warning); font-size: 1.05rem;">"JESTEŚ NA WAITLIŚCIE (KOLEJKA)"</h4>
                                        <p style="font-size: 0.8rem; color: var(--text-muted);">
                                            "Brak wolnych miejsc w tej chwili. Kiedy ktoś spóźni się z płatnością, system automatycznie przydzieli Ci miejsce."
                                        </p>
                                    </div>
                                </div>
                                <button
                                    class="btn btn-secondary"
                                    on:click=move |_| {
                                        waitlisted_events.update(|set| {
                                            set.remove(&evt_id2);
                                        });
                                        add_toast("Opuszczono waitlistę.".to_string(), ToastType::Warning);
                                    }
                                >
                                    "Opuść kolejkę"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        ().into_view()
                    }
                } else {
                    ().into_view()
                }}

                // General Actions Block
                <div class="glass-card action-bar">
                    <div class="action-info">
                        <h4>{move || selected_event.get().map(|e| e.name.clone()).unwrap_or_default()}</h4>
                        <p>
                            "Organizator: " {move || selected_event.get().map(|e| e.venue.clone()).unwrap_or_default()} " | Status: "
                            {move || if let Some(evt) = selected_event.get() {
                                if evt.available_seats > 0 {
                                    format!("Dostępne {} biletów", evt.available_seats)
                                } else {
                                    "Brak wolnych biletów. Dostępny zapis do kolejki.".to_string()
                                }
                            } else { "".to_string() }}
                        </p>
                    </div>

                    {move || {
                        let handle_reserve = handle_reserve_for_click.clone();
                        if active_hold.get().is_none() && selected_event.get().map(|evt| !waitlisted_events.get().contains(&evt.id)).unwrap_or(false) {
                            if let Some(evt) = selected_event.get() {
                                let is_full = evt.available_seats == 0;
                                view! {
                                    <button
                                        class=move || if is_full { "btn btn-secondary" } else { "btn" }
                                        disabled=move || is_reserving.get()
                                        on:click=move |_| handle_reserve()
                                    >
                                        {move || if is_reserving.get() {
                                            view! { <div class="spinner"></div> }.into_view()
                                        } else if is_full {
                                            "Zapisz się do kolejki (Waitlist)".into_view()
                                        } else {
                                            "Zarezerwuj wolny bilet".into_view()
                                        }}
                                    </button>
                                }.into_view()
                            } else {
                                ().into_view()
                            }
                        } else {
                            ().into_view()
                        }
                    }}
                </div>

                // Seat Map
                <div class="glass-card">
                    <div class="seat-map-header">
                        <h3 style="font-size: 1.1rem;">"Plan sali / miejsca"</h3>

                        <div class="seat-legends">
                            <div class="legend-item">
                                <div class="legend-dot dot-available"></div>
                                <span>"Wolne"</span>
                            </div>
                            <div class="legend-item">
                                <div class="legend-dot dot-held-by-me"></div>
                                <span>"Twój hold"</span>
                            </div>
                            <div class="legend-item">
                                <div class="legend-dot dot-held"></div>
                                <span>"Zarezerwowane (hold)"</span>
                            </div>
                            <div class="legend-item">
                                <div class="legend-dot dot-sold"></div>
                                <span>"Kupione"</span>
                            </div>
                        </div>
                    </div>

                    // Inline seat grid
                    <div class="seat-grid-container">
                        <div class="seat-grid">
                            {move || {
                                let all_seats = seats.get();
                                let uid = user_id.get();
                                let has_hold = active_hold.get().is_some();
                                let on_waitlist = selected_event_id.get()
                                    .map(|eid| waitlisted_events.get().contains(&eid))
                                    .unwrap_or(false);
                                let handle_reserve = handle_reserve_for_click.clone();

                                all_seats.into_iter().map(move |seat| {
                                    let seat_status = seat.status.clone();
                                    let seat_label = seat.label.clone().unwrap_or_default();
                                    let is_held_by_me = seat_status == "held" && seat.held_by_user_id == Some(uid);

                                    let seat_class = match seat_status.as_str() {
                                        "available" => "seat seat-available",
                                        "held" if is_held_by_me => "seat seat-held-by-me",
                                        "held" => "seat seat-held",
                                        "sold" => "seat seat-sold",
                                        _ => "seat",
                                    };

                                    let on_seat_click = {
                                        let seat_status = seat_status.clone();
                                        let handle_reserve = handle_reserve.clone();
                                        move |_| {
                                            if seat_status == "available"
                                                && !has_hold
                                                && !on_waitlist
                                            {
                                                handle_reserve();
                                            }
                                        }
                                    };

                                    let label_display = seat_label.clone();
                                    view! {
                                        <div class=seat_class on:click=on_seat_click>
                                            {label_display}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>

        // Toasts
        <div class="toast-container">
            {move || toasts.get().into_iter().map(|t| {
                let class_name = match t.toast_type {
                    ToastType::Success => "toast toast-success",
                    ToastType::Error => "toast toast-error",
                    ToastType::Warning => "toast toast-warning",
                };
                view! {
                    <div class=class_name>
                        <span>{t.message}</span>
                    </div>
                }
            }).collect::<Vec<_>>() }
        </div>
    }
}
