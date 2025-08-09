INSERT INTO firms (id, name)
VALUES ('a6a7572a-5553-4653-a733-35a0b602790f', 'Default Firm')
ON CONFLICT (id) DO NOTHING;

INSERT INTO clients (id, firm_id, company_name, email)
VALUES ('e2b1c3d4-5f6a-7b8c-9d0e-f1a2b3c4d5e6', 'a6a7572a-5553-4653-a733-35a0b602790f', 'Default Client', 'default@client.com')
ON CONFLICT (id) DO NOTHING;

INSERT INTO users (id, firm_id, first_name, last_name, email, password_hash)
VALUES ('b1c2d3e4-5f6a-7b8c-9d0e-f1a2b3c4d5e6', 'a6a7572a-5553-4653-a733-35a0b602790f', 'Default', 'User', 'user@email.com', '$2b$10$eImiTMZG4T5WjZz1a1a1uO3h5d6f7g8h9i0j1k2l3m4n5o6p7q8r9')
ON CONFLICT (id) DO NOTHING;

INSERT INTO collections (id, client_id, user_id, title, status, access_token, expires_at)
VALUES ('c1d2e3f4-5a6b-7c8d-9e0f-a1b2c3d4e5f6', 'e2b1c3d4-5f6a-7b8c-9d0e-f1a2b3c4d5e6', 'b1c2d3e4-5f6a-7b8c-9d0e-f1a2b3c4d5e6', 'Default Collection', 'active', 'access_token_example', NOW() + INTERVAL '1 hour')
ON CONFLICT (id) DO NOTHING;

INSERT INTO requests (id, collection_id, title, description, status)
VALUES ('d1e2f3a4-5b6c-7d8e-9f0a-b1c2d3e4f5f6', 'c1d2e3f4-5a6b-7c8d-9e0f-a1b2c3d4e5f6', 'Default Request', 'This is a default request description.', 'pending')
ON CONFLICT (id) DO NOTHING;
