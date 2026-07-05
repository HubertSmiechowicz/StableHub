import { invoke } from "@tauri-apps/api/core";
import type { CreateHorseRequest, HorseDetails, HorseSummary } from "../types/horse";

export function listHorses() {
  return invoke<HorseSummary[]>("list_horses");
}

export function createHorse(request: CreateHorseRequest) {
  return invoke<HorseDetails>("create_horse", { request });
}

export function getHorseDetails(id: string) {
  return invoke<HorseDetails>("get_horse_details", { id });
}
