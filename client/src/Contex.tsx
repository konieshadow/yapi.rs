import React from "react";
import { UserInfo } from "./types/UserInfo";

export const UserContext = React.createContext<UserInfo | undefined>(undefined);
