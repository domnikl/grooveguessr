import { Paper } from "@mui/material";

type ElevatedPaperProps = {
  children: React.ReactNode;
};

export default function ElevatedPaper(props: ElevatedPaperProps) {
  return (
    <Paper elevation={4} sx={{ padding: "20px" }}>
      {props.children}
    </Paper>
  );
}
