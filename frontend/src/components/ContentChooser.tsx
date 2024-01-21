import { gql, useMutation } from "@apollo/client";
import { FormControl, Stack, TextField, Typography } from "@mui/material";
import { useState } from "react";
import { validateUrl } from "../utils";
import ReactPlayer from "react-player";
import { set } from "date-fns";

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
  disabled: boolean;
};

export default function ContentChooser(props: ContentChooserProps) {
  const [url, setUrl] = useState<string>(props.defaultUrl ?? "");
  const [error, setError] = useState<string | null>(null);
  const [setContent] = useMutation(SET_CONTENT);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    let newUrl: string | null = event.target.value?.trim() ?? null;
    let saveUrl: string | null = newUrl;

    if (newUrl.length === 0) {
      setError(null);
      newUrl = null;
    } else if (!validateUrl(newUrl)) {
      setError("Invalid URL");
      saveUrl = null;
    }

    setUrl(newUrl ?? "");

    // TODO: debounce

    setContent({
      variables: {
        id: props.lobbyId,
        url: saveUrl,
      },
    })
      .catch((error) => {
        setError(error.message);
      })
      .then(() => {
        setError(null);
      });
  };

  return (
    <Stack>
      <Typography variant="h3">Choose a video</Typography>

      <Typography>
        Paste a YouTube link below to choose a video. The video will be played
        to everyone to guess.
      </Typography>

      <Stack>
        <FormControl>
          <TextField
            rows={1}
            autoFocus={true}
            disabled={props.disabled}
            value={url}
            onChange={handleChange}
            variant="filled"
            type="url"
            id="youtubeUrl"
            error={error !== null}
            helperText={error}
          />
        </FormControl>

        {url && !error && (
          <ReactPlayer
            width={580}
            url={url ?? ""}
            onError={() => {
              setError("Can't load video");
            }}
          />
        )}
      </Stack>
    </Stack>
  );
}
