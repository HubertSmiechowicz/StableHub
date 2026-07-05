CREATE TABLE IF NOT EXISTS calendar_entries (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    scheduled_on TEXT NOT NULL,
    scheduled_time TEXT NULL,
    entry_type TEXT NOT NULL,
    description TEXT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived_at TEXT NULL
);

CREATE INDEX IF NOT EXISTS idx_calendar_entries_status ON calendar_entries(status);
CREATE INDEX IF NOT EXISTS idx_calendar_entries_scheduled_on ON calendar_entries(scheduled_on);
CREATE INDEX IF NOT EXISTS idx_calendar_entries_type ON calendar_entries(entry_type);
