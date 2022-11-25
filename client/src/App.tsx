import React, { FC } from 'react'
import {
  RouterProvider,
  Route,
  createBrowserRouter,
} from 'react-router-dom';
import Home from './containers/Home/Home';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />,
  },
  {
    path: '/hello',
    element: <div>Hello, world!</div>
  },
]);

const App: FC = () => (
  <RouterProvider router={router} />
)

export default App
