import { Avatar, Stack } from "@mui/material";
import { stringToColor } from "../utils";

type UserAvatarProps = {
  name: string;
};

export default function UserAvatar(props: UserAvatarProps) {
  return (
    <Stack direction="row" alignItems="center">
      <Avatar sx={{ backgroundColor: stringToColor(props.name) }}>
        {props.name[0].toUpperCase()}
      </Avatar>
      {props.name}
    </Stack>
  );
}
