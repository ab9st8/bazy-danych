CREATE EXTENSION IF NOT EXISTS "pgcrypto"; -- enables uuid

CREATE TABLE events (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT        NOT NULL,
    date        DATE        NOT NULL,
    venue       TEXT        NOT NULL,
    total_seats INT         NOT NULL CHECK (total_seats > 0)
);

CREATE TABLE seats (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id        UUID        NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    label           TEXT,
    status          TEXT        NOT NULL DEFAULT 'available'
                        CHECK (status IN ('available', 'held', 'sold')),
    held_until      TIMESTAMPTZ,
    held_by_user_id UUID,

    UNIQUE (event_id, label)
);

CREATE TABLE reservations (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seat_id      UUID        NOT NULL REFERENCES seats(id) ON DELETE CASCADE,
    user_id      UUID        NOT NULL,
    confirmed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE waitlist (
    id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id  UUID        NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    user_id   UUID        NOT NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_seats_event_status   ON seats(event_id, status);
CREATE INDEX idx_waitlist_event_joined ON waitlist(event_id, joined_at);
CREATE INDEX idx_reservations_seat    ON reservations(seat_id);
