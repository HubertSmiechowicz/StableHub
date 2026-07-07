ALTER TABLE inventory_items
ADD COLUMN last_usage_applied_at TEXT;

UPDATE inventory_items
SET last_usage_applied_at = updated_at
WHERE last_usage_applied_at IS NULL;

CREATE TABLE IF NOT EXISTS inventory_stocktakes (
    id TEXT PRIMARY KEY,
    inventory_item_id TEXT NOT NULL,
    counted_on TEXT NOT NULL,
    expected_quantity REAL NOT NULL,
    actual_quantity REAL NOT NULL,
    variance_quantity REAL NOT NULL,
    expected_usage REAL NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (inventory_item_id) REFERENCES inventory_items(id)
);

CREATE INDEX IF NOT EXISTS idx_inventory_stocktakes_item_id
    ON inventory_stocktakes(inventory_item_id);

CREATE INDEX IF NOT EXISTS idx_inventory_stocktakes_counted_on
    ON inventory_stocktakes(counted_on);
