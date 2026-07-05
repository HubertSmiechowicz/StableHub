import { invoke } from "@tauri-apps/api/core";
import type {
  CreateInventoryItemRequest,
  InventoryItemDetails,
  InventoryItemSummary
} from "../types/inventory";

export function listInventoryItems() {
  return invoke<InventoryItemSummary[]>("list_inventory_items");
}

export function createInventoryItem(request: CreateInventoryItemRequest) {
  return invoke<InventoryItemDetails>("create_inventory_item", { request });
}

export function getInventoryItemDetails(id: string) {
  return invoke<InventoryItemDetails>("get_inventory_item_details", { id });
}
