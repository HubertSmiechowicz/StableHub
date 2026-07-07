CREATE TABLE IF NOT EXISTS inventory_deliveries (
    id TEXT PRIMARY KEY,
    inventory_item_id TEXT NOT NULL,
    delivered_on TEXT NOT NULL,
    quantity REAL NOT NULL,
    total_cost REAL NOT NULL,
    supplier TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (inventory_item_id) REFERENCES inventory_items(id)
);

CREATE INDEX IF NOT EXISTS idx_inventory_deliveries_item_id
    ON inventory_deliveries(inventory_item_id);

CREATE INDEX IF NOT EXISTS idx_inventory_deliveries_delivered_on
    ON inventory_deliveries(delivered_on);
