CREATE TABLE IF NOT EXISTS services (
    id           SERIAL PRIMARY KEY,
    name         TEXT NOT NULL,
    description  TEXT NOT NULL,
    price_from   DOUBLE PRECISION NOT NULL,
    duration_hrs INTEGER NOT NULL DEFAULT 2,
    icon         TEXT NOT NULL DEFAULT 'bug',
    popular      BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS bookings (
    id             SERIAL PRIMARY KEY,
    full_name      TEXT NOT NULL,
    email          TEXT NOT NULL,
    phone          TEXT NOT NULL,
    address        TEXT NOT NULL,
    service_id     INTEGER NOT NULL REFERENCES services(id),
    service_name   TEXT NOT NULL,
    preferred_date DATE NOT NULL,
    preferred_time TEXT NOT NULL,
    notes          TEXT,
    status         TEXT NOT NULL DEFAULT 'pending',
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS contact_messages (
    id         SERIAL PRIMARY KEY,
    full_name  TEXT NOT NULL,
    email      TEXT NOT NULL,
    phone      TEXT,
    subject    TEXT NOT NULL,
    message    TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO services (name, description, price_from, duration_hrs, icon, popular) VALUES
('Residential Fumigation', 'Complete fumigation of your home, all rooms and hidden areas treated.', 15000, 3, 'home', true),
('Commercial Fumigation',  'Large-scale treatment for offices, warehouses and shops.',              45000, 6, 'building', false),
('Termite Treatment',      'Targeted elimination using baiting and liquid barrier systems.',        20000, 4, 'bug', true),
('Rodent Control',         'Safe removal of rats and mice with trapping and exclusion.',            12000, 2, 'mouse', false),
('Bed Bug Treatment',      'Heat treatment and chemical application to eliminate bed bugs.',        18000, 4, 'bed', false),
('Mosquito Control',       'Fogging and residual spraying to eliminate mosquito breeding grounds.',  8000, 2, 'droplet', false)
ON CONFLICT DO NOTHING;