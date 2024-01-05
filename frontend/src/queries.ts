import { gql } from "@apollo/client";

export const GET_LOBBY = gql`
  query getLobby($id: String!) {
    lobby(lobbyInput: { id: $id }) {
      id
      guessingTime
      startedAt
      createdAt
    }
  }
`;
