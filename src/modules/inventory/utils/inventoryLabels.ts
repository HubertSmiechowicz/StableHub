const unitLabels: Record<string, string> = {
  piece: "szt.",
  kg: "kg",
  bale: "kostka",
  liter: "l",
  bag: "worek",
  package: "opakowanie"
};

export const inventoryUnitOptions = [
  { value: "piece", label: "szt." },
  { value: "kg", label: "kg" },
  { value: "bale", label: "kostka" },
  { value: "liter", label: "l" },
  { value: "bag", label: "worek" },
  { value: "package", label: "opakowanie" }
];

export function unitLabel(value: string) {
  return unitLabels[value] ?? value;
}

export function formatQuantity(value: number, unit: string) {
  return `${formatNumber(value)} ${unitLabel(unit)}`;
}

export function formatDays(value: number | null) {
  if (value === null) {
    return "Nie obliczono";
  }

  return `${formatNumber(value)} dni`;
}

export function formatOptionalQuantity(value: number | null, unit: string) {
  if (value === null) {
    return "Nie podano";
  }

  return formatQuantity(value, unit);
}

export function formatCurrency(value: number) {
  return new Intl.NumberFormat("pl-PL", {
    currency: "PLN",
    maximumFractionDigits: 2,
    style: "currency"
  }).format(value);
}

export function formatOptionalCurrency(value: number | null) {
  if (value === null) {
    return "Nie obliczono";
  }

  return formatCurrency(value);
}

export function formatNumber(value: number) {
  return new Intl.NumberFormat("pl-PL", {
    maximumFractionDigits: 2
  }).format(value);
}
