export function formatTime(value: number): String {
  const minutes = Math.floor(value / 60);
  const seconds = value % 60;

  return `${minutes.toString().padStart(2, "0")}:${seconds
    .toString()
    .padStart(2, "0")}`;
}

export function validateUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch (e) {
    return false;
  }
}

export function stringToColor(value: string): string {
  const hash = value.split("").reduce((acc, char) => {
    acc = (acc << 5) - acc + char.toUpperCase().charCodeAt(0);
    return acc & acc;
  }, 0);

  const hue = Math.abs(hash % 360);
  const lightness = 50 + Math.abs(hash % 20);

  return `hsl(${hue}, 60%, ${lightness}%)`;
}
