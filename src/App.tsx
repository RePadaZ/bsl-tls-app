import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {AppState} from "./Type.ts";
import LoadingScreen from "./components/LoadingScreen.tsx";
import ErrorScreen from "./components/ErrorScreen.tsx";
import MainScreen from "./components/MainScreen.tsx";

export function App() {
  const [state, setState] = useState<AppState>("loading");
  const [errorMessage, setErrorMessage] = useState<string>("");

  const checkApp = async () => {
    try {
      await invoke("validate_bsl_path");
      setState("ready");
    } catch (err) {
      setErrorMessage(String(err));
      setState("error");
    }
  };

  useEffect(() => {
    checkApp().then(r => setErrorMessage(String(r)));
  }, []);

  switch (state) {
    case "loading":
      return <LoadingScreen />;
    case "error":
      return <ErrorScreen message={errorMessage} onRetry={checkApp} />;
    case "ready":
      return <MainScreen />;
  }
}
