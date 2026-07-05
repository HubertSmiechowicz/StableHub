CREATE TABLE IF NOT EXISTS health_events (
    id TEXT PRIMARY KEY,
    horse_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    occurred_on TEXT NOT NULL,
    title TEXT NOT NULL,
    notes TEXT NULL,
    cost REAL NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT NULL,
    FOREIGN KEY (horse_id) REFERENCES horses(id)
);

CREATE INDEX IF NOT EXISTS idx_health_events_horse_id ON health_events(horse_id);
CREATE INDEX IF NOT EXISTS idx_health_events_type ON health_events(event_type);
CREATE INDEX IF NOT EXISTS idx_health_events_status ON health_events(status);
CREATE INDEX IF NOT EXISTS idx_health_events_occurred_on ON health_events(occurred_on);
