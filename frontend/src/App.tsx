import { RouterProvider, createHashRouter } from "react-router-dom";
import Layout from "./Layout";
import Home from "./pages/Home";
import { ThemeProvider, createTheme } from "@mui/material";
import { LocalizationProvider } from "@mui/x-date-pickers";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";
import Lobby, { loader as lobbyLoader } from "./pages/Lobby";
import ErrorBoundary from "./pages/ErrorBoundary";

const router = createHashRouter([
  {
    path: "/",
    Component: Layout,
    errorElement: <ErrorBoundary error={null} />,
    children: [
      { path: "/game/:id", Component: Lobby, loader: lobbyLoader },
      { path: "/", Component: Home },
    ],
  },
]);

export default function App() {
  const theme = createTheme({
    palette: {
      primary: {
        main: "#6489E8",
      },
      mode: "dark",
    },
    typography: {
      h1: {
        fontFamily: "Moirai One",
        fontSize: "2.5rem",
      },
      h2: {
        fontFamily: "Moirai One",
        fontSize: "1.5rem",
      },
    },
  });

  return (
    <ThemeProvider theme={theme}>
      <LocalizationProvider dateAdapter={AdapterDateFns}>
        <RouterProvider router={router} />
      </LocalizationProvider>
    </ThemeProvider>
  );
}
