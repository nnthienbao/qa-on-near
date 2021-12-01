import "regenerator-runtime/runtime";
import React, { useState, useEffect } from "react";
import { Routes, Route, Link, Switch } from 'react-router-dom';
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Button from "@mui/material/Button";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import Grid from "@mui/material/Grid";
import { login, logout } from "./utils";
import HomePage from "./pages/HomePage";
import DetailQuestionPage from "./pages/DetailQuestionPage";

import getConfig from "./config";
const { networkId } = getConfig(process.env.NODE_ENV || "development");

export default function App({ walletConnection, accountId, contract }) {
  const [isSignIn, setIsSignIn] = useState(false);
  useEffect(() => {
    // in this case, we only care to query the contract when signed in
    if (walletConnection.isSignedIn()) {
      setIsSignIn(true);
    }
  }, []);
  return (
    <>
      <AppBar position="static">
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            <h3>Q&A on NEAR</h3>
          </Typography>
          {isSignIn ? (
            <Button color="inherit" onClick={logout}>
              Logout
            </Button>
          ) : (
            <Button color="inherit" onClick={login}>
              Login
            </Button>
          )}
        </Toolbar>
      </AppBar>
      <Grid
        container
        spacing={2}
        direction="column"
        alignItems="center"
        justifyContent="center"
      >
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/detail" element={<DetailQuestionPage />} />
        </Routes>
      </Grid>
    </>
  );
}
