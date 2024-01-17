import { Container, Stack } from "@mui/material";
import ReactPlayer from "react-player";
import { useEffect, useState } from "react";
import { Lobby } from "../model/Lobby";
import { formatTime } from "../utils";

type GameLoopProps = {
  lobby: Lobby;
};

export default function GameLoop(props: GameLoopProps) {
  const [timer, setTimer] = useState<number>(props.lobby.guessingTime); // TODO: set default value from lobby

  // TODO: Game timer
  // TODO: in which round are we currently?

  useEffect(() => {
    const interval = setInterval(() => {
      if (timer > 0) {
        setTimer((timer) => timer - 1);
      } else {
        clearInterval(interval);
      }
    }, 1000);
  }, [props.lobby, timer]);

  return (
    <Container maxWidth="sm">
      <Stack>
        <Stack direction="row" justifyContent="flex-end">
          {formatTime(timer)}
        </Stack>
        <ReactPlayer url="https://www.youtube.com/watch?v=LXb3EKWsInQ" />
      </Stack>
    </Container>
  );
}
