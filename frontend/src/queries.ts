import { gql } from "@apollo/client";

export const GET_LOBBY = gql`
  query getLobby($id: String!) {
    lobby(id: $id) {
      id
      guessingTime
      startedAt
      createdAt
      host {
        id
        name
      }
      players {
        id
        name
        isReady
      }
      content {
        data
        type
      }
      currentContent {
        data
        type
      }
      guesses
      roundIndex
    }
    profile {
      id
      name
    }
  }
`;
