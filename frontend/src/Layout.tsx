import "regenerator-runtime/runtime.js";
import CssBaseline from "@mui/material/CssBaseline";
import Box from "@mui/material/Box";
import { AppBar, Container, Toolbar, Typography } from "@mui/material";
import { Outlet, useNavigate } from "react-router-dom";

export default function Layout() {
  const navigate = useNavigate();

  return (
    <div>
      <CssBaseline />
      <Box sx={{ flexGrow: 1 }}>
        <AppBar position="fixed">
          <Toolbar>
            <Typography
              variant="h6"
              component="div"
              sx={{ flexGrow: 1, cursor: "pointer" }}
              onClick={() => navigate("/")}
            >
              grooveguessr
            </Typography>
          </Toolbar>
        </AppBar>
      </Box>

      <Container sx={{ marginTop: "70px" }} maxWidth={false}>
        <Outlet />
      </Container>
    </div>
  );
}
