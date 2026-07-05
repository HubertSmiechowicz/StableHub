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
  created_at: string;
  updated_at: string;
  archived_at: string | null;
};

export type CreateInventoryItemRequest = {
  name: string;
  unit: string;
  quantity: number;
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
