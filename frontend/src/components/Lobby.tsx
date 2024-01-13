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

const JOIN_LOBBY = gql`
  mutation joinLobby($id: String!) {
    joinLobby(id: $id) {
      id
    }
  }
`;

const SET_READY = gql`
  mutation setReady($id: String!, $ready: Boolean!) {
    setReady(id: $id, ready: $ready) {
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
  const [setReady] = useMutation(SET_READY);

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
  }, [props.data, isDirty, joinLobby]);

  const handleChangeCommitted = (guessingTime: number) => {
    if (props.data?.lobby?.host?.id !== props.data?.profile?.id) {
      // only host can update game settings
      return;
    }

    configureLobby({
      variables: { id: props.data?.lobby?.id, guessingTime },
    }).then(() => {
      setIsDirty(false);
      props.startPolling();
    });
  };

  const player = props.data?.lobby?.players?.filter(
    (p: Player) => p.id === props.data?.profile?.id
  )[0];
  const isHost = props.data?.lobby?.host?.id === player?.id;
  const everyoneReady = props.data?.lobby?.players?.every(
    (p: Player) => p.isReady
  );
  const numberOfPlayers = props.data?.lobby?.players?.length;

  return (
    <Container maxWidth="sm">
      <IsLoading isLoading={props.isLoading}>
        <Stack spacing={4} direction="row">
          <Stack spacing={4}>
            <Typography variant="h1">
              {props.data?.lobby?.host?.name}'s Lobby
            </Typography>

            <Typography variant="h2">Lobby Settings</Typography>

            <Typography variant="h2">Game Settings</Typography>

            <FormControl>
              <GuessingTimeSlider
                ariaLabel="Guessing Time"
                defaultValue={120}
                min={10}
                max={3 * 60}
                disabled={!isHost}
                guessingTime={guessingTime ?? 120}
                onChangeCommitted={handleChangeCommitted}
                onChange={(guessingTime) => {
                  props.stopPolling();
                  setGuessingTime(guessingTime);
                }}
              />
            </FormControl>
          </Stack>

          <Players players={props.data?.lobby?.players} />
        </Stack>

        <Stack direction="row">
          <Button
            size="large"
            variant="contained"
            onClick={() => {
              props.stopPolling();

              setReady({
                variables: {
                  id: props.data?.lobby?.id,
                  ready: !player?.isReady,
                },
              }).then(() => {
                props.startPolling();
              });
            }}
          >
            {!player?.isReady ? "Ready" : "Not Ready"}
          </Button>

          {isHost && everyoneReady && numberOfPlayers >= 3 && (
            <Button
              size="large"
              variant="contained"
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
          )}
        </Stack>
      </IsLoading>
    </Container>
  );
}
