import { gql } from "@apollo/client";

export const GET_LOBBY = gql`
  query getLobby($id: String!) {
    lobby(id: $id) {
      id
      guessingTime
      startedAt
      createdAt
      hostId
    }
    profile {
      id
      name
    }
  }
`;
