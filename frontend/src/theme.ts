import { ThemeOptions } from "@mui/material/styles";

export const themeOptions: ThemeOptions = {
  palette: {
    mode: "dark",
    primary: {
      main: "#ff8f00",
    },
    secondary: {
      main: "#e0e0e0",
    },
    background: {
      default: "#1a237e",
      paper: "#283593",
    },
  },
  typography: {
    h1: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "2.5rem",
    },
    h2: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.75rem",
    },
    h3: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.5rem",
    },
    h4: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.25rem",
    },
    h5: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.2rem",
    },
    h6: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.15rem",
    },
  },
  components: {
    MuiStack: {
      defaultProps: {
        spacing: 2,
        useFlexGap: true,
      },
    },
  },
};
