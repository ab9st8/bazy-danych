-- insert example events
INSERT INTO events (id, name, date, venue, total_seats)
VALUES (
        'a0000000-0000-0000-0000-000000000001',
        'Zenon Martyniuk LIVE TOUR 2026',
        '2026-07-15',
        'Tauron Arena Kraków',
        3000
    ),
    (
        'a0000000-0000-0000-0000-000000000002',
        'Multi-agent Claude Code - od podstaw',
        '2026-08-08',
        'Dom Spokojnej Starości Jaworzno',
        400
    ),
    (
        'a0000000-0000-0000-0000-000000000003',
        'Mistrzostwa kraju w sudoku wyczynowym (miejsca obserwatorskie)',
        '2026-09-20',
        'Młodzieżowy Dom Kultury Kraków',
        50
    ),
    (
        'a0000000-0000-0000-0000-000000000004',
        'Konsultacje do trzeciego terminu egzaminu z Matematyki dyskretnej',
        '2026-02-20',
        'AGH D-10',
        10
    ) ON CONFLICT DO NOTHING;
-- insert free seats per event
INSERT INTO seats (event_id, label, status)
SELECT 'a0000000-0000-0000-0000-000000000001'::uuid,
    i::text,
    'available'
FROM generate_series(1, 3000) AS i ON CONFLICT DO NOTHING;
INSERT INTO seats (event_id, label, status)
SELECT 'a0000000-0000-0000-0000-000000000002'::uuid,
    i::text,
    'available'
FROM generate_series(1, 400) AS i ON CONFLICT DO NOTHING;
INSERT INTO seats (event_id, label, status)
SELECT 'a0000000-0000-0000-0000-000000000003'::uuid,
    i::text,
    'available'
FROM generate_series(1, 50) AS i ON CONFLICT DO NOTHING;
INSERT INTO seats (event_id, label, status)
SELECT 'a0000000-0000-0000-0000-000000000004'::uuid,
    i::text,
    'available'
FROM generate_series(1, 10) AS i ON CONFLICT DO NOTHING;