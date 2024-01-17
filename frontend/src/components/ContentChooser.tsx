import { gql, useMutation } from "@apollo/client";
import { FormControl, Stack, TextField, Typography } from "@mui/material";
import { useState } from "react";
import { validateUrl } from "../utils";

export const SET_CONTENT = gql`
  mutation setContent($id: String!, $url: String!) {
    setContent(id: $id, url: $url) {
      content {
        data
      }
    }
  }
`;

type ContentChooserProps = {
  lobbyId: string;
  defaultUrl?: string;
  onContentSelected: (content: any) => void;
};

export default function ContentChooser(props: ContentChooserProps) {
  const [url, setUrl] = useState<String>(props.defaultUrl ?? "");
  const [error, setError] = useState<String | null>(null);
  const [setContent] = useMutation(SET_CONTENT);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const newUrl = event.target.value;
    setUrl(newUrl);

    if (newUrl.length === 0) {
      setError(null);
      return;
    } else if (!validateUrl(newUrl)) {
      setError("Invalid URL");
      return;
    }

    // TODO: debounce

    setContent({
      variables: {
        id: props.lobbyId,
        url: newUrl,
      },
    })
      .catch((error) => {
        setError(error.message);
      })
      .then((result: any) => {
        setUrl(result.data.setContent.content.data);
        setError(null);
        props.onContentSelected(newUrl);
      });
  };

  return (
    <Stack>
      <Typography variant="h3">Select a video</Typography>

      <Typography>
        Choose your content to guess. Must be a valid YouTube link.
      </Typography>

      <Stack>
        <FormControl>
          <TextField
            rows={1}
            autoFocus={true}
            defaultValue={props.defaultUrl}
            value={url}
            onChange={handleChange}
            variant="filled"
            type="url"
            id="youtubeUrl"
            error={error !== null}
            helperText={error}
          />
        </FormControl>
      </Stack>
    </Stack>
  );
}
