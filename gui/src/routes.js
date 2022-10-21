import Home from "pages/index";
import { createBrowserRouter } from "react-router-dom";
import PageFrame from "components/frame";
import Pics from "pages/pics";

const config = [
    {
        path: '/',
        element: <PageFrame />,
        children: [
            {
                path: '',
                element: <Home />,
            },
            {
                path: '/pic',
                element: <Pics />,
            },
        ]
    }
];

export default createBrowserRouter(config);