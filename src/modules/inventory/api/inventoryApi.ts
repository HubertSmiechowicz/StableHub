import { invoke } from "@tauri-apps/api/core";
import type {
  CreateInventoryItemRequest,
  InventoryItemDetails,
  InventoryItemSummary,
  InventoryListRequest,
  UpdateInventoryItemRequest
} from "../types/inventory";

export function listInventoryItems(request: InventoryListRequest) {
  return invoke<InventoryItemSummary[]>("list_inventory_items", { request });
}

export function createInventoryItem(request: CreateInventoryItemRequest) {
  return invoke<InventoryItemDetails>("create_inventory_item", { request });
}

export function updateInventoryItem(request: UpdateInventoryItemRequest) {
  return invoke<InventoryItemDetails>("update_inventory_item", { request });
}

export function archiveInventoryItem(id: string) {
  return invoke<void>("archive_inventory_item", { id });
}

export function getInventoryItemDetails(id: string) {
  return invoke<InventoryItemDetails>("get_inventory_item_details", { id });
}
