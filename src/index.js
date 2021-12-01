import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter as Router } from "react-router-dom";
import App from "./App";
import { initContract } from "./utils";

window.nearInitPromise = initContract()
  .then(() => {
    ReactDOM.render(
      <Router>
        <App
          walletConnection={window.walletConnection}
          accountId={window.accountId}
          contract={window.contract}
        />
      </Router>,
      document.querySelector("#root")
    );
  })
  .catch(console.error);
