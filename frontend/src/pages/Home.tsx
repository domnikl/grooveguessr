import { gql, useMutation } from "@apollo/client";
import {
  Box,
  Button,
  Container,
  FormControl,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import IsLoading from "../components/IsLoading";
import { useNavigate } from "react-router-dom";
import { useErrorBoundary } from "react-error-boundary";
import { useState } from "react";

export const CREATE_LOBBY = gql`
  mutation createLobby {
    createLobby {
      id
    }
  }
`;

export default function Home() {
  const navigate = useNavigate();
  const { showBoundary } = useErrorBoundary();
  const [createLobby, { loading }] = useMutation(CREATE_LOBBY);
  const [joinGameId, setJoinGameId] = useState<string>("");

  const hostGame = () => {
    createLobby().then((res) => {
      if (res.errors) {
        showBoundary(res.errors);
      } else {
        navigate(`/game/${res.data.createLobby.id}`);
      }
    });
  };

  const joinGame = () => {
    if (joinGameId.length === 0) {
      return;
    }

    // TODO: check if this is a valid game id before redirecting

    navigate(`/game/${joinGameId}`);
  };

  return (
    <Container maxWidth="sm">
      <IsLoading isLoading={loading}>
        <Stack spacing={{ xs: 3, sm: 5 }}>
          <Typography variant="h2">GrooveGuessr</Typography>

          <p>
            Welcom to GrooveGuessr, a fun game where you guess your friends
            taste in (music) videos.
          </p>

          <Button variant="contained" onClick={() => hostGame()}>
            Host a game
          </Button>

          <Box sx={{ textAlign: "center" }}>or</Box>

          <Stack>
            <FormControl>
              <TextField
                value={joinGameId}
                onChange={(e) => setJoinGameId(e.target.value)}
                label="Game ID"
                variant="outlined"
              />
            </FormControl>
            <FormControl>
              <Button variant="contained" onClick={() => joinGame()}>
                join game
              </Button>
            </FormControl>
          </Stack>
        </Stack>
      </IsLoading>
    </Container>
  );
}
