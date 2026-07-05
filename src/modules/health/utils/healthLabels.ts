export const healthEventTypeOptions = [
  { value: "vaccination", label: "Szczepienie" },
  { value: "deworming", label: "Odrobaczanie" },
  { value: "vet_visit", label: "Weterynarz" },
  { value: "farrier", label: "Kowal" },
  { value: "treatment", label: "Leczenie" },
  { value: "checkup", label: "Kontrola" },
  { value: "other", label: "Inne" }
];

export function healthEventTypeLabel(value: string) {
  return healthEventTypeOptions.find((option) => option.value === value)?.label ?? value;
}

export function displayValue(value: string | null | undefined) {
  return value && value.trim().length > 0 ? value : "nie podano";
}

export function formatCost(value: number | null | undefined) {
  if (value === null || value === undefined) {
    return "nie podano";
  }

  return `${value.toFixed(2)} zł`;
}
