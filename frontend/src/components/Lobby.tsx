import { Button, Container, Stack, Typography } from "@mui/material";

import { gql, useMutation } from "@apollo/client";
import IsLoading from "./IsLoading";
import { useEffect, useState } from "react";
import GuessingTimeSlider from "./GuessingTimeSlider";
import Players from "./Players";
import { Player } from "../model/Player";
import ContentChooser from "./ContentChooser";
import ElevatedPaper from "./ElevatedPaper";

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
  isHost: boolean;
  stopPolling: () => void;
  startPolling: () => void;
};

export default function Lobby(props: LobbyProps) {
  const [configureLobby] = useMutation(CONFIGURE_LOBBY);
  const [startGame] = useMutation(START_GAME);
  const [setReady] = useMutation(SET_READY);

  const [isDirty, setIsDirty] = useState<boolean>(false);
  const [guessingTime, setGuessingTime] = useState<number>(
    props.data?.lobby?.guessingTime ?? 120
  );

  const hasVideo = props.data?.lobby?.content?.data;

  useEffect(() => {
    if (isDirty) {
      // don't overwrite if there are pending changes
      return;
    }

    setGuessingTime(props.data?.lobby?.guessingTime);
  }, [props.data, isDirty]);

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
  const everyoneReady = props.data?.lobby?.players?.every(
    (p: Player) => p.isReady
  );
  const numberOfPlayers = props.data?.lobby?.players?.length;
  //const readyToStart = everyoneReady && numberOfPlayers >= 3;
  const readyToStart = everyoneReady; // TODO: change back to above

  let readyCaption = "Ready";
  let readyColor: "success" | "error" = "success";

  if (player?.isReady) {
    readyCaption = "Not Ready";
    readyColor = "error";
  }

  return (
    <Container maxWidth="md">
      <IsLoading isLoading={props.isLoading}>
        <Typography
          variant="h1"
          sx={{ marginBottom: "50px", marginTop: "100px" }}
        >
          {props.data?.lobby?.host?.name}'s Lobby
        </Typography>

        <Stack spacing={4} direction="row">
          <Stack spacing={4}>
            <ElevatedPaper>
              <Typography variant="h2">Game Settings</Typography>

              <GuessingTimeSlider
                ariaLabel="Guessing Time"
                defaultValue={120}
                min={10}
                max={3 * 60}
                disabled={!props.isHost}
                guessingTime={guessingTime ?? 120}
                onChangeCommitted={handleChangeCommitted}
                onChange={(guessingTime) => {
                  props.stopPolling();
                  setGuessingTime(guessingTime);
                }}
              />
            </ElevatedPaper>

            <ElevatedPaper>
              <ContentChooser
                disabled={player?.isReady}
                lobbyId={props.data?.lobby?.id}
                defaultUrl={props.data?.lobby?.content?.data}
              />
            </ElevatedPaper>
          </Stack>

          <Stack>
            <Players
              players={props.data?.lobby?.players}
              editableId={props.data?.profile?.id}
            />

            <Stack justifyItems="stretch">
              <Button
                size="large"
                variant="contained"
                color={readyColor}
                disabled={!hasVideo}
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
                {readyCaption}
              </Button>

              {props.isHost && readyToStart && (
                <Button
                  size="large"
                  variant="contained"
                  color="success"
                  onClick={() => {
                    props.stopPolling();
                    startGame({
                      variables: { id: props.data?.lobby?.id },
                    }).then(() => {
                      props.startPolling();
                    });
                  }}
                >
                  Start Game
                </Button>
              )}
            </Stack>
          </Stack>
        </Stack>
      </IsLoading>
    </Container>
  );
}
