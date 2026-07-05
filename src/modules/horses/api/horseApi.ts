import { invoke } from "@tauri-apps/api/core";
import type {
  CreateHorseRequest,
  HorseDetails,
  HorseListRequest,
  HorseSummary,
  UpdateHorseRequest
} from "../types/horse";

export function listHorses(request: HorseListRequest) {
  return invoke<HorseSummary[]>("list_horses", { request });
}

export function createHorse(request: CreateHorseRequest) {
  return invoke<HorseDetails>("create_horse", { request });
}

export function updateHorse(request: UpdateHorseRequest) {
  return invoke<HorseDetails>("update_horse", { request });
}

export function archiveHorse(id: string) {
  return invoke<void>("archive_horse", { id });
}

export function getHorseDetails(id: string) {
  return invoke<HorseDetails>("get_horse_details", { id });
}
