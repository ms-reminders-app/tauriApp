import React from "react";
import ReactDOM from "react-dom/client";
import NotificationApp from "./NotificationApp.tsx";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <NotificationApp />
  </React.StrictMode>,
);
