import { ThemeOptions } from "@mui/material/styles";

export const themeOptions: ThemeOptions = {
  palette: {
    mode: "dark",
    primary: {
      main: "#F2F2F2",
    },
    secondary: {
      main: "#E9DFF2",
    },
    background: {
      default: "#463B8C",
      paper: "#1B1640",
    },
  },
  typography: {
    h1: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "2.5rem",
      fontWeight: 900,
      color: "#E9DFF2",
    },
    h2: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.75rem",
      fontWeight: 800,
      color: "#CEB3F2",
    },
    h3: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.5rem",
      fontWeight: 700,
      color: "#CEB3F2",
    },
    h4: {
      fontFamily: '"Moirai One", "Roboto", "Helvetica", "Arial", sans-serif',
      fontSize: "1.25rem",
      fontWeight: 600,
      color: "#CEB3F2",
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
