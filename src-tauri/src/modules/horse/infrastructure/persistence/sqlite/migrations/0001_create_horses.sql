CREATE TABLE IF NOT EXISTS horses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    sex TEXT NULL,
    breed TEXT NULL,
    date_of_birth TEXT NULL,
    coat_color TEXT NULL,
    identification_number TEXT NULL,
    notes TEXT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT NULL
);

CREATE INDEX IF NOT EXISTS idx_horses_status ON horses(status);
CREATE INDEX IF NOT EXISTS idx_horses_name ON horses(name);
