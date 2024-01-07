import { ApolloError } from "@apollo/client/errors";
import { useNavigate } from "react-router-dom";

export default function ErrorBoundary({ error }: { error: any }) {
  const navigate = useNavigate();

  if (typeof error === "string") {
    return <h1>{error}</h1>;
  } else if (error instanceof ApolloError) {
    if (error.extraInfo) {
      return <h1>Network error.</h1>;
    } else if (
      error.graphQLErrors.map((e) => e.extensions.code === 404).length > 0
    ) {
      return <h1>Not Found.</h1>;
    } else if (
      error.graphQLErrors.map((e) => e.extensions.code === 401).length > 0
    ) {
      navigate("/login");
    } else if (error.extraInfo) return <h1>{error.message}</h1>;
  } else if (error instanceof Error) {
    return <h1>{error.message}</h1>;
  }

  return <h1>Something went wrong.</h1>;
}
