import {
  Button,
  Container,
  FormControl,
  Stack,
  Typography,
} from "@mui/material";

import { gql, useMutation } from "@apollo/client";
import IsLoading from "./IsLoading";
import { useEffect, useState } from "react";
import GuessingTimeSlider from "./GuessingTimeSlider";
import Players from "./Players";
import { Player } from "../model/Player";

export const CONFIGURE_LOBBY = gql`
  mutation configureLobby($id: String!, $guessingTime: Int!) {
    configureLobby(id: $id, guessingTime: $guessingTime) {
      id
    }
  }
`;

export const START_GAME = gql`
  mutation startGame($id: String!) {
    startGame(id: $id) {
      id
    }
  }
`;

export const JOIN_LOBBY = gql`
  mutation joinLobby($id: String!) {
    joinLobby(id: $id) {
      id
    }
  }
`;

type LobbyProps = {
  id: string;
  isLoading: boolean;
  data: any;
  stopPolling: () => void;
  startPolling: () => void;
};

export default function Lobby(props: LobbyProps) {
  const [configureLobby] = useMutation(CONFIGURE_LOBBY);
  const [startGame] = useMutation(START_GAME);
  const [joinLobby] = useMutation(JOIN_LOBBY);

  const [isDirty, setIsDirty] = useState<boolean>(false);
  const [guessingTime, setGuessingTime] = useState<number>(
    props.data?.lobby?.guessingTime ?? 120
  );

  useEffect(() => {
    // if the user is not yet part of the lobby, join it
    const playerIds = props.data?.lobby?.players?.map((p: Player) => p.id);

    if (playerIds.indexOf(props.data?.profile?.id) === -1) {
      joinLobby({ variables: { id: props.data?.lobby?.id } });
    }

    if (isDirty) {
      // don't overwrite if there are pending changes
      return;
    }

    setGuessingTime(props.data?.lobby?.guessingTime);
  }, [props.data, isDirty]);

  const handleChangeCommitted = (guessingTime: number) => {
    if (props.data?.lobby?.hostId !== props.data?.profile?.id) {
      // only host can update game settings
      return;
    }

    console.log({ message: "handleChangeCommitted", guessingTime });

    configureLobby({
      variables: { id: props.data?.lobby?.id, guessingTime },
    }).then(() => {
      setIsDirty(false);
      props.startPolling();
    });
  };

  return (
    <Container maxWidth="sm">
      <IsLoading isLoading={props.isLoading}>
        <Stack spacing={2}>
          <Typography variant="h1">{props.data?.lobby?.id}</Typography>
          <Players players={props.data?.lobby?.players} />

          <Typography variant="h2">Settings</Typography>
          <FormControl>
            <GuessingTimeSlider
              ariaLabel="Guessing Time"
              defaultValue={120}
              min={10}
              max={4 * 60}
              disabled={props.data?.lobby?.hostId !== props.data?.profile?.id}
              guessingTime={guessingTime ?? 120}
              onChangeCommitted={handleChangeCommitted}
              onChange={(guessingTime) => {
                props.stopPolling();
                setGuessingTime(guessingTime);
              }}
            />
          </FormControl>
          <FormControl>
            <Typography variant="h2">Start Game</Typography>
            <Button
              onClick={() => {
                props.stopPolling();
                startGame({ variables: { id: props.data?.lobby?.id } }).then(
                  () => {
                    props.startPolling();
                  }
                );
              }}
            >
              Start Game
            </Button>
          </FormControl>
        </Stack>
      </IsLoading>
    </Container>
  );
}
