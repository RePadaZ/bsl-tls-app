import Main_screen from "./pages/main_screen.tsx";
import {BrowserRouter, Route, Routes} from "react-router";
import {Setting_screen} from "./pages/setting_screen.tsx";

export function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Main_screen/>}/>
                <Route path="/setting" element={<Setting_screen/>}/>
            </Routes>
        </BrowserRouter>
    );
}