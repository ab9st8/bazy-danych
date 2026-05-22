use leptos::*;
use crate::api::Seat;
use uuid::Uuid;

#[component]
pub fn SeatGrid(
    seats: RwSignal<Vec<Seat>>,
    user_id: RwSignal<Uuid>,
    active_hold: Memo<Option<Seat>>,
    #[prop(into)] is_waitlisted: Signal<bool>,
    handle_reserve: impl Fn() + Clone + 'static,
) -> impl IntoView {
    let handle_reserve_clone = handle_reserve.clone();

    view! {
        <div class="seat-grid-container">
            <div class="seat-grid">
                {move || {
                    let all_seats = seats.get();
                    let uid = user_id.get();
                    let has_hold = active_hold.get().is_some();
                    let on_waitlist = is_waitlisted.get();
                    let handle_reserve = handle_reserve_clone.clone();

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

                        let label_for_title = seat_label.clone();
                        let status_for_title = seat_status.clone();
                        let is_mine = is_held_by_me;

                        view! {
                            <div class=seat_class on:click=on_seat_click title=move || {
                                let status_text = match status_for_title.as_str() {
                                    "available" => "Wolne (Kliknij, aby zarezerwować)".to_string(),
                                    "held" if is_mine => "Twój hold (Kliknij 'Opłać bilet')".to_string(),
                                    "held" => "Tymczasowa rezerwacja".to_string(),
                                    "sold" => "Sprzedane".to_string(),
                                    _ => "".to_string(),
                                };
                                format!("Seat #{} - {}", label_for_title, status_text)
                            }>
                                {seat_label.clone()}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
