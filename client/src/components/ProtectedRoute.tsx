import { FC, ReactElement, useContext } from "react";
import { Navigate, Outlet } from "react-router-dom";
import { UserContext } from "../Contex";

interface Props {
  children?: ReactElement<any, any> | null;
}

const ProtectedRoute: FC<Props> = props => {
  const userInfo = useContext(UserContext);
  if (!userInfo) {
    return <Navigate to="/" replace />;
  }

  return props.children ? props.children : <Outlet />;
};

export default ProtectedRoute;
