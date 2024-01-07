import { useLoaderData } from "react-router-dom";
import { GET_LOBBY } from "../queries";
import { useQuery } from "@apollo/client";
import Lobby from "../components/Lobby";
import { useErrorBoundary } from "react-error-boundary";
import IsLoading from "../components/IsLoading";
import GameLoop from "../components/GameLoop";

export async function loader({ params }: { params: any }) {
  return params.id;
}

export default function Game() {
  const { showBoundary } = useErrorBoundary();
  const loaderData = useLoaderData() as string;

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

  if (!data) {
    return <IsLoading isLoading={true} />;
  } else if (data.lobby?.startedAt !== null) {
    return <GameLoop lobby={data.lobby} />;
  } else {
    return (
      <Lobby
        id={loaderData}
        data={data}
        isLoading={loading}
        startPolling={() => startPolling(500)}
        stopPolling={stopPolling}
      />
    );
  }
}
