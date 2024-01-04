import { RouterProvider, createHashRouter } from "react-router-dom";
import Layout from "./Layout";
import Home from "./pages/Home";
import { ThemeProvider, createTheme } from "@mui/material";
import { LocalizationProvider } from "@mui/x-date-pickers";
import { AdapterDateFns } from "@mui/x-date-pickers/AdapterDateFns";

const router = createHashRouter([
  {
    path: "/",
    Component: Layout,
    children: [{ path: "/", Component: Home }],
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
  });

  return (
    <ThemeProvider theme={theme}>
      <LocalizationProvider dateAdapter={AdapterDateFns}>
        <RouterProvider router={router} />
      </LocalizationProvider>
    </ThemeProvider>
  );
}
