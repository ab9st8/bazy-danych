use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BASE_URL: &str = "http://localhost:3000";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub date: String,
    pub venue: String,
    pub total_seats: i32,
    pub available_seats: i64,
    pub held_seats: i64,
    pub sold_seats: i64,
    pub waitlist_count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Seat {
    pub id: Uuid,
    pub label: Option<String>,
    pub status: String, // "available", "held", "sold"
    pub held_until_epoch: Option<f64>,
    pub held_by_user_id: Option<Uuid>,
    pub reservation_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ReserveRequest {
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReservationResponse {
    pub reservation_id: Uuid,
    pub seat_id: Uuid,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WaitlistResponse {
    pub waitlist_id: Uuid,
}

#[derive(Serialize)]
pub struct PaymentRequest {
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PaymentResponse {
    pub reservation_id: Uuid,
    pub seat_id: Uuid,
}

pub enum ReserveResult {
    Reserved(ReservationResponse),
    Waitlisted(WaitlistResponse),
}

pub async fn fetch_events() -> Result<Vec<Event>, String> {
    let url = format!("{}/events", BASE_URL);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json::<Vec<Event>>()
        .await
        .map_err(|e| format!("Failed to parse events: {}", e))
}

pub async fn fetch_seats(event_id: Uuid) -> Result<Vec<Seat>, String> {
    let url = format!("{}/events/{}/seats", BASE_URL, event_id);
    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json::<Vec<Seat>>()
        .await
        .map_err(|e| format!("Failed to parse seats: {}", e))
}

pub async fn reserve_seat(event_id: Uuid, user_id: Uuid) -> Result<ReserveResult, String> {
    let url = format!("{}/events/{}/reservations", BASE_URL, event_id);
    let req = ReserveRequest { user_id };

    let response = Request::post(&url)
        .json(&req)
        .map_err(|e| format!("Serialize error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    match response.status() {
        201 => {
            let body = response
                .json::<ReservationResponse>()
                .await
                .map_err(|e| format!("Parse reservation response error: {}", e))?;
            Ok(ReserveResult::Reserved(body))
        }
        202 => {
            let body = response
                .json::<WaitlistResponse>()
                .await
                .map_err(|e| format!("Parse waitlist response error: {}", e))?;
            Ok(ReserveResult::Waitlisted(body))
        }
        status => {
            let text = response.text().await.unwrap_or_default();
            Err(format!("Error reservation status {}: {}", status, text))
        }
    }
}

pub async fn pay_reservation(reservation_id: Uuid, user_id: Uuid) -> Result<PaymentResponse, String> {
    let url = format!("{}/reservations/{}/payment", BASE_URL, reservation_id);
    let req = PaymentRequest { user_id };

    let response = Request::post(&url)
        .json(&req)
        .map_err(|e| format!("Serialize error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.status() == 200 {
        response
            .json::<PaymentResponse>()
            .await
            .map_err(|e| format!("Parse payment response error: {}", e))
    } else {
        let text = response.text().await.unwrap_or_default();
        Err(format!("Payment failed ({}) - {}", response.status(), text))
    }
}
