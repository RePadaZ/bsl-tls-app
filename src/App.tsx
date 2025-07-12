import Main_screen from "./pages/main_screen.tsx";
import {BrowserRouter, Route, Routes} from "react-router";
import {Setting_screen} from "./pages/setting_screen.tsx";
import Result_page from "./pages/result_page.tsx";

export function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Main_screen/>}/>
                <Route path="/setting" element={<Setting_screen/>}/>
                <Route path="/result" element={<Result_page/>}/>
            </Routes>
        </BrowserRouter>
    );
}