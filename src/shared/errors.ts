export function formatError(caught: unknown, fallback: string) {
  if (typeof caught === "string" && caught.trim().length > 0) {
    return caught;
  }

  if (caught instanceof Error && caught.message.trim().length > 0) {
    return caught.message;
  }

  return fallback;
}
