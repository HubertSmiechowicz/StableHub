export function sexLabel(value: string | null) {
  switch (value) {
    case "mare":
      return "Klacz";
    case "stallion":
      return "Ogier";
    case "gelding":
      return "Wałach";
    default:
      return "Nie podano";
  }
}

export function displayValue(value: string | null) {
  return value && value.trim().length > 0 ? value : "Nie podano";
}
