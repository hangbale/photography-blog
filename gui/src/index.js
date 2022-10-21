import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import './index.css';
import routes from './routes';
import { NextUIProvider } from '@nextui-org/react';
import 'bootstrap-icons/font/bootstrap-icons.css'
const root = ReactDOM.createRoot(document.getElementById('root'));


root.render(
     <NextUIProvider>
        <RouterProvider router={routes} />
     </NextUIProvider>
);