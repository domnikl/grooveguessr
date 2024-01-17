import { Badge, Stack, Typography } from "@mui/material";
import { Player } from "../model/Player";
import { Check, Close } from "@mui/icons-material";

type PlayersProps = {
  players: Player[];
};

export default function Players(props: PlayersProps) {
  return (
    <Stack spacing={{ xs: 3, sm: 5 }}>
      <Typography variant="h2">Players</Typography>

      {props.players.map((player: Player) =>
        player.isReady ? (
          <Badge badgeContent={<Check />} color="success" key={player.id}>
            <Typography key={player.id}>{player.name}</Typography>
          </Badge>
        ) : (
          <Badge badgeContent={<Close />} color="error" key={player.id}>
            <Typography key={player.id}>{player.name}</Typography>
          </Badge>
        )
      )}
    </Stack>
  );
}
