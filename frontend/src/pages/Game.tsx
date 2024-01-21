import { useLoaderData } from "react-router-dom";
import { GET_LOBBY } from "../queries";
import { gql, useMutation, useQuery } from "@apollo/client";
import Lobby from "../components/Lobby";
import { useErrorBoundary } from "react-error-boundary";
import IsLoading from "../components/IsLoading";
import GameLoop from "../components/GameLoop";
import { Player } from "../model/Player";

const JOIN_LOBBY = gql`
  mutation joinLobby($id: String!) {
    joinLobby(id: $id) {
      id
    }
  }
`;

export async function loader({ params }: { params: any }) {
  return params.id;
}

export default function Game() {
  const { showBoundary } = useErrorBoundary();
  const loaderData = useLoaderData() as string;
  const [joinLobby] = useMutation(JOIN_LOBBY);

  const { loading, data, error, startPolling, stopPolling } = useQuery(
    GET_LOBBY,
    {
      variables: { id: loaderData },
      pollInterval: 500,
    }
  );

  if (error) {
    showBoundary(error);
  }

  const player = data?.lobby?.players?.filter(
    (p: Player) => p.id === data?.profile?.id
  )[0];
  const isHost = data?.lobby?.host?.id === player?.id;
  const playerIds = data?.lobby?.players?.map((p: Player) => p.id);

  // if the user is not yet part of the lobby, join it
  if (playerIds?.indexOf(data?.profile?.id) === -1) {
    joinLobby({ variables: { id: data?.lobby?.id } });
  }

  if (!data) {
    return <IsLoading isLoading={true} />;
  } else if (data.lobby?.startedAt !== null) {
    return <GameLoop lobby={data.lobby} isHost={isHost} />;
  } else {
    return (
      <Lobby
        id={loaderData}
        data={data}
        isLoading={loading}
        isHost={isHost}
        startPolling={() => startPolling(500)}
        stopPolling={stopPolling}
      />
    );
  }
}
