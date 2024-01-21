import {
  Avatar,
  Badge,
  Button,
  Chip,
  Container,
  Stack,
  Typography,
} from "@mui/material";
import ReactPlayer from "react-player";
import { useState } from "react";
import { Lobby } from "../model/Lobby";
import { formatTime } from "../utils";
import useTimer from "../useTimer";
import { gql, useMutation } from "@apollo/client";
import { Check, FastForward } from "@mui/icons-material";
import UserAvatar from "./UserAvatar";

export const GUESS = gql`
  mutation guess($id: String!, $roundIndex: Int!, $guessedUserId: String!) {
    guess(id: $id, roundIndex: $roundIndex, guessedUserId: $guessedUserId) {
      id
    }
  }
`;

export const FORWARD = gql`
  mutation forward($id: String!) {
    forward(id: $id) {
      id
    }
  }
`;

type GameLoopProps = {
  lobby: Lobby;
  isHost: boolean;
};

export default function GameLoop(props: GameLoopProps) {
  const [guess] = useMutation(GUESS);
  const [forward] = useMutation(FORWARD);
  const [videoFinished, setVideoFinished] = useState<boolean>(false);

  const { start: startTimer, remaining } = useTimer(
    props.lobby.guessingTime,
    () => {
      setVideoFinished(true);
    }
  );

  // TODO: display round number
  // TODO: prevent user from skipping video
  // TODO: sound effect when timer comes to an end
  // TODO: display which user was guessed

  const handleGuess = (playerId: string) => {
    if (props.lobby.roundIndex === null) {
      return;
    }

    guess({
      variables: {
        id: props.lobby.id,
        roundIndex: props.lobby.roundIndex,
        guessedUserId: playerId,
      },
    });
  };

  let guessed_id: String | null = null;

  if (props.lobby.roundIndex !== null && props.lobby.guesses) {
    guessed_id = props.lobby.guesses[props.lobby.roundIndex];
  }

  let forwardCaption = (
    <>
      Forward <FastForward />
    </>
  );

  if (props.lobby.roundIndex === props.lobby.players.length - 1) {
    forwardCaption = <>Results</>;
  }

  const handleForward = () => {
    setVideoFinished(true);

    if (props.lobby.roundIndex === props.lobby.players.length - 1) {
      return;
    }

    forward({
      variables: {
        id: props.lobby.id,
      },
    });
  };

  return (
    <Container maxWidth="sm">
      <Stack>
        <Stack direction="row" justifyContent="space-between">
          <Stack>
            <Typography variant="h1">
              Round {(props.lobby?.roundIndex ?? 0) + 1}
            </Typography>
          </Stack>
          <Stack>
            <Typography variant="h1">{formatTime(remaining)}</Typography>
          </Stack>
        </Stack>

        {!videoFinished && (
          <ReactPlayer
            url={props.lobby.currentContent?.data}
            playing={!videoFinished}
            playsinline={true}
            config={{
              youtube: {
                playerVars: {
                  showinfo: 0,
                  controls: 0,
                  disablekb: 1,
                  fs: 0,
                  end: props.lobby.guessingTime,
                },
              },
            }}
            onReady={() => {
              startTimer();
              setVideoFinished(false);
            }}
            onProgress={(progress) => {
              // TODO: check back later when seek is implemented to offset it
              if (progress.playedSeconds >= props.lobby.guessingTime) {
                setVideoFinished(true);
              }
            }}
            onPause={() => {
              if (remaining > 0) {
                setVideoFinished(false);
              }
            }}
          />
        )}
        {videoFinished && (
          <Stack>
            <Typography variant="h1">Round over</Typography>
            <Typography variant="h3">Who was that from?</Typography>
            <Typography>Guess who that video was from.</Typography>

            <Stack>
              {props.lobby.players.map((player) => (
                <Stack
                  direction="row"
                  alignItems="center"
                  justifyContent="space-between"
                  key={player.id}
                  sx={{
                    cursor: "pointer",
                    backgroundColor: (theme) => {
                      return guessed_id === player.id
                        ? theme.palette.success.dark
                        : "";
                    },
                    borderRadius: "10px",
                    padding: "10px",
                  }}
                  onClick={() => {
                    handleGuess(player.id);
                  }}
                >
                  <UserAvatar name={player.name} />
                  {guessed_id === player.id && (
                    <Chip label="Guessed" color="success" />
                  )}
                </Stack>
              ))}
            </Stack>

            {props.isHost && (
              <Button
                variant="contained"
                color="success"
                onClick={handleForward}
              >
                {forwardCaption}
              </Button>
            )}
          </Stack>
        )}
      </Stack>
    </Container>
  );
}
