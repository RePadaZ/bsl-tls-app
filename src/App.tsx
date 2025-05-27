import MainScreen from "./pages/MainScreen.tsx";
import {BrowserRouter, Route, Routes} from "react-router";
import {SettingScreen} from "./pages/SettingScreen.tsx";

export function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<MainScreen/>}/>
                <Route path="/setting" element={<SettingScreen/>}/>
            </Routes>
        </BrowserRouter>
    );
}