import { invoke } from "@tauri-apps/api/core";
import type {
  CreateHealthEventRequest,
  HealthEventDetails,
  HealthEventSummary,
  HealthListRequest,
  UpdateHealthEventRequest
} from "../types/health";

export function listHealthEvents(request: HealthListRequest) {
  return invoke<HealthEventSummary[]>("list_health_events", { request });
}

export function createHealthEvent(request: CreateHealthEventRequest) {
  return invoke<HealthEventDetails>("create_health_event", { request });
}

export function updateHealthEvent(request: UpdateHealthEventRequest) {
  return invoke<HealthEventDetails>("update_health_event", { request });
}

export function archiveHealthEvent(id: string) {
  return invoke<void>("archive_health_event", { id });
}

export function getHealthEventDetails(id: string) {
  return invoke<HealthEventDetails>("get_health_event_details", { id });
}
