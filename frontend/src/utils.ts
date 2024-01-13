export function formatTime(value: number): String {
  const minutes = Math.floor(value / 60);
  const seconds = value % 60;

  return `${minutes.toString().padStart(2, "0")}:${seconds
    .toString()
    .padStart(2, "0")}`;
}
