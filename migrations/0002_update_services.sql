UPDATE services SET price_from = 25000 WHERE name = 'Residential Fumigation';
UPDATE services SET price_from = 50000 WHERE name = 'Commercial Fumigation';
UPDATE services SET price_from = 50000 WHERE name = 'Bed Bug Treatment';
UPDATE services SET price_from = 25000 WHERE name = 'Mosquito Control';
UPDATE services SET price_from = 50000 WHERE name = 'Termite Treatment';

INSERT INTO services (name, description, price_from, duration_hrs, icon, popular)
VALUES ('Reptile Control', 'Safe and humane removal of lizards, snakes and other reptiles from your property.', 50000, 3, 'bug', false)
ON CONFLICT DO NOTHING;
