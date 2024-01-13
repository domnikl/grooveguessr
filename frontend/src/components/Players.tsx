import { Badge, Stack, Typography } from "@mui/material";
import { Player } from "../model/Player";
import { Check } from "@mui/icons-material";

type PlayersProps = {
  players: Player[];
};

export default function Players(props: PlayersProps) {
  return (
    <Stack spacing={{ xs: 3, sm: 5 }}>
      <Typography variant="h2">Players</Typography>

      {props.players.map((player: Player) =>
        player.isReady ? (
          <Badge badgeContent={<Check />} color="primary" key={player.id}>
            <Typography key={player.id}>{player.name}</Typography>
          </Badge>
        ) : (
          <Typography key={player.id}>{player.name}</Typography>
        )
      )}
    </Stack>
  );
}
