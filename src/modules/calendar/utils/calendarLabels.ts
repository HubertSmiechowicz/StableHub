import { healthEventTypeLabel } from "../../health/utils/healthLabels";

export const calendarEntryTypeOptions = [
  { value: "task", label: "Zadanie" },
  { value: "reminder", label: "Przypomnienie" },
  { value: "visit", label: "Wizyta" },
  { value: "payment", label: "Płatność" },
  { value: "other", label: "Inne" }
];

export function calendarEntryTypeLabel(value: string) {
  return calendarEntryTypeOptions.find((option) => option.value === value)?.label ?? value;
}

export function calendarItemTypeLabel(sourceModule: string, itemType: string) {
  if (sourceModule === "health") {
    return healthEventTypeLabel(itemType);
  }

  return calendarEntryTypeLabel(itemType);
}

export function sourceModuleLabel(value: string) {
  if (value === "calendar") {
    return "Terminarz";
  }

  if (value === "health") {
    return "Zdrowie";
  }

  return value;
}

export function displayValue(value: string | null | undefined) {
  return value && value.trim().length > 0 ? value : "nie podano";
}
