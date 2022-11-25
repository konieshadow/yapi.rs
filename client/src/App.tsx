import React, { FC } from 'react'
import {
  RouterProvider,
  Route,
  createBrowserRouter,
} from 'react-router-dom';
import Footer, { defaultFootList } from './components/Footer/Footer';
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
  <div className="g-main">
    <div className="router-main">
      <div className="router-container">
      <RouterProvider router={router} />
      </div>
    </div>
    <Footer footList={defaultFootList} />
  </div>
)

export default App
