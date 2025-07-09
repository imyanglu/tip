import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Process from "./pages/Process";
import "./global.css";
import { BrowserRouter, Route, Routes } from "react-router";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/process" element={<Process />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);
