import React from "react";
import {createBrowserRouter,
  RouterProvider } from 'react-router-dom';
import "./styles/App.css";
import Home from "./Home";
import About from "./about"


const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/about",
    element: <About />,
  }
]);

function App() {
  return ( 
    <RouterProvider router={router} />
  );
}

export default App;
