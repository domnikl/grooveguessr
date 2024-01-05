import { Container, Stack } from "@mui/material";
import { GET_LOBBY } from "../queries";
import { useQuery } from "@apollo/client";
import { useLoaderData } from "react-router-dom";
import IsLoading from "../components/IsLoading";
import { useErrorBoundary } from "react-error-boundary";

export async function loader({ params }: { params: any }) {
  return params.id;
}

export default function Lobby() {
  const { showBoundary } = useErrorBoundary();
  const loaderData = useLoaderData() as string;

  const { loading, data, error } = useQuery(GET_LOBBY, {
    variables: { id: loaderData },
  });

  if (error) {
    showBoundary(error);
  }

  return (
    <Container maxWidth="sm">
      <IsLoading isLoading={loading}>
        <Stack>
          <h1>{data?.lobby?.id}</h1>
        </Stack>
      </IsLoading>
    </Container>
  );
}
