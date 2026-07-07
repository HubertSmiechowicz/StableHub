export type InventoryItemSummary = {
  id: string;
  name: string;
  unit: string;
  quantity: number;
  minimum_quantity: number | null;
  daily_usage: number | null;
  days_remaining: number | null;
  status: string;
};

export type InventoryItemDetails = InventoryItemSummary & {
  recent_deliveries: InventoryDeliverySummary[];
  recent_stocktakes: InventoryStocktakeSummary[];
  total_delivery_cost: number;
  average_unit_cost: number | null;
  last_usage_applied_at: string | null;
  pending_usage_days: number;
  pending_usage_quantity: number;
  created_at: string;
  updated_at: string;
  archived_at: string | null;
};

export type InventoryDeliverySummary = {
  id: string;
  inventory_item_id: string;
  delivered_on: string;
  quantity: number;
  total_cost: number;
  unit_cost: number | null;
  supplier: string | null;
  notes: string | null;
  created_at: string;
};

export type InventoryStocktakeSummary = {
  id: string;
  inventory_item_id: string;
  counted_on: string;
  expected_quantity: number;
  actual_quantity: number;
  variance_quantity: number;
  expected_usage: number;
  notes: string | null;
  created_at: string;
};

export type CreateInventoryItemRequest = {
  name: string;
  unit: string;
  minimum_quantity: number | null;
  daily_usage: number | null;
};

export type UpdateInventoryItemRequest = CreateInventoryItemRequest & {
  id: string;
};

export type InventoryListRequest = {
  search: string | null;
  sort_by: "name" | "quantity" | "days_remaining" | "created_at";
  sort_direction: "asc" | "desc";
};

export type RegisterInventoryDeliveryRequest = {
  inventory_item_id: string;
  delivered_on: string;
  quantity: number;
  total_cost: number;
  supplier: string | null;
  notes: string | null;
};

export type RecordInventoryStocktakeRequest = {
  inventory_item_id: string;
  counted_on: string;
  actual_quantity: number;
  notes: string | null;
};
