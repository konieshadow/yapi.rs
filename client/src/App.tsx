import { useRequest } from 'ahooks';
import { FC } from 'react'
import {
  RouterProvider,
  createBrowserRouter,
} from 'react-router-dom';
import { userStatus } from './api/user';
import Footer, { defaultFootList } from './components/Footer/Footer';
import ProtectedRoute from './components/ProtectedRoute';
import GrroupList from './containers/Group/GroupList/GroupList';
import Home from './containers/Home/Home';
import LoginContainer from './containers/Login/LoginContailer';
import { UserContext } from './Contex';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Home />,
  },
  {
    path: '/login',
    element: <LoginContainer />
  },
  {
    path: '/group',
    element: (
      <ProtectedRoute>
        <GrroupList />
      </ProtectedRoute>
    )
  }
]);

const App: FC = () => {
  const { data, loading } = useRequest(userStatus);

  return (
    <div className="g-main">
      <UserContext.Provider value={data}>
        {
          loading ? <></> : <>
            <div className="router-main">
              <div className="router-container">
                <RouterProvider router={router} />
              </div>
            </div>
            <Footer footList={defaultFootList} />
          </>
        }
      </UserContext.Provider>
    </div>
  )
};

export default App;
