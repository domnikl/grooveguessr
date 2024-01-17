import { Badge, Button, Stack, TextField, Typography } from "@mui/material";
import { Player } from "../model/Player";
import { Check, Close } from "@mui/icons-material";
import { gql, useMutation } from "@apollo/client";
import { useState } from "react";

export const SET_NAME = gql`
  mutation setName($name: String!) {
    setName(name: $name) {
      name
    }
  }
`;

type PlayersProps = {
  players: Player[];
  editableId: string;
};

function PlayerName(props: { player: Player; editable: boolean }) {
  const [setName] = useMutation(SET_NAME);
  const [editName, setEditName] = useState<string | null>(null);
  const [editError, setEditError] = useState<string | null>(null);

  let icon = <Close />;
  let color: "error" | "success" = "error";

  if (props.player?.isReady) {
    icon = <Check />;
    color = "success";
  }

  let cursor = "default";

  if (props.editable) {
    cursor = "pointer";
  }

  const isValidName = (name: string | null) => {
    return name !== null && name.length > 2;
  };

  const startEditing = () => {
    if (!props.editable) {
      return;
    }

    setEditError(null);
    setEditName(props.player.name);
  };

  const cancelEditing = () => {
    setEditName(null);
    setEditError(null);
  };

  const finishEditing = () => {
    if (!props.editable) {
      return;
    }

    if (isValidName(editName)) {
      setEditError("Name cannot be empty");
    }

    setName({
      variables: {
        name: editName,
      },
    })
      .catch((error) => {
        setEditError(error.message);
      })
      .finally(() => {
        setEditName(null);
        setEditError(null);
      });
  };

  return (
    <Badge badgeContent={icon} color={color}>
      <Stack direction="row">
        {props.editable && editName !== null ? (
          <Stack>
            <TextField
              value={editName}
              variant="filled"
              onBlur={cancelEditing}
              error={editError !== null}
              helperText={editError}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  finishEditing();
                  e.preventDefault();
                }

                if (e.key === "Escape") {
                  cancelEditing();
                  e.preventDefault();
                }
              }}
              onChange={(e) => {
                setEditName(e.target.value);

                if (!isValidName(e.target.value)) {
                  setEditError("Name cannot be empty");
                } else {
                  setEditError(null);
                }
              }}
            />
            <Button variant="contained" color="success" onClick={finishEditing}>
              <Check />
            </Button>
          </Stack>
        ) : (
          <Typography
            sx={{ cursor: cursor }}
            key={props.player.id}
            onClick={startEditing}
          >
            {props.player.name}
          </Typography>
        )}
      </Stack>
    </Badge>
  );
}

export default function Players(props: PlayersProps) {
  return (
    <Stack spacing={{ xs: 3, sm: 5 }}>
      <Typography variant="h2">Players</Typography>

      {props.players.map((player: Player) => (
        <PlayerName
          player={player}
          key={player.id}
          editable={player.id === props.editableId}
        />
      ))}
    </Stack>
  );
}
