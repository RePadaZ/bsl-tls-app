import {invoke} from "@tauri-apps/api/core";
import type {AppState} from "./Type.ts";
import LoadingScreen from "./pages/LoadingScreen.tsx";
import ErrorScreen from "./pages/ErrorScreen.tsx";
import MainScreen from "./pages/MainScreen.tsx";
import {BrowserRouter, Route, Routes} from "react-router";
import {useState} from "react";
import {SettingScreen} from "./pages/SettingScreen.tsx";

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

    return (
        <BrowserRouter>
            <Routes>
                {state.valueOf() === "loading" && <Route path="*" element={<LoadingScreen/>}/>}
                {state.valueOf() === "error" &&
                    <Route path="*" element={<ErrorScreen message={errorMessage} onRetry={checkApp}/>}/>}
                {state.valueOf() === "ready" && <Route path="*" element={<MainScreen/>}/>}

                <Route path={'/'} element={<MainScreen/>}/>
                <Route path={'/error'} element={<ErrorScreen message={errorMessage} onRetry={checkApp}/>}/>
                <Route path={'/loading'} element={<LoadingScreen/>}/>
                <Route path={'/setting'} element={<SettingScreen/>}/>

            </Routes>
        </BrowserRouter>
    )
}