import { invoke } from "@tauri-apps/api/core";
import type {
  CalendarEntryDetails,
  CalendarItemSummary,
  CalendarListRequest,
  CreateCalendarEntryRequest,
  UpdateCalendarEntryRequest
} from "../types/calendar";

export function listCalendarItems(request: CalendarListRequest) {
  return invoke<CalendarItemSummary[]>("list_calendar_items", { request });
}

export function createCalendarEntry(request: CreateCalendarEntryRequest) {
  return invoke<CalendarEntryDetails>("create_calendar_entry", { request });
}

export function updateCalendarEntry(request: UpdateCalendarEntryRequest) {
  return invoke<CalendarEntryDetails>("update_calendar_entry", { request });
}

export function archiveCalendarEntry(id: string) {
  return invoke<void>("archive_calendar_entry", { id });
}

export function getCalendarEntryDetails(id: string) {
  return invoke<CalendarEntryDetails>("get_calendar_entry_details", { id });
}
