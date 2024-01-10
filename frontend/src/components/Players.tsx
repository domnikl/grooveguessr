import { Stack, Typography } from "@mui/material";
import { Player } from "../model/Player";

type PlayersProps = {
  players: Player[];
};

export default function Players(props: PlayersProps) {
  return (
    <>
      <Typography variant="h2">Players</Typography>
      <Stack spacing={{ xs: 3, sm: 5 }}>
        {props.players.map((player: Player) => (
          <Typography key={player.id}>{player.name}</Typography>
        ))}
      </Stack>
    </>
  );
}
