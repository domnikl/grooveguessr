import { Content } from "./Content";
import { Player } from "./Player";

export type Lobby = {
  id: string;
  guessingTime: number;
  roundIndex: number | null;
  guesses: String[] | null;
  currentContent: null | Content;
  content: null | Content;
  players: Player[];
};
