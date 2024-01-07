import { CircularProgress } from "@mui/material";

type IsLoadingProps = {
  isLoading: boolean;
  children?: any | undefined;
};

export default function IsLoading(props: IsLoadingProps) {
  return props.isLoading ? <CircularProgress /> : props.children;
}
