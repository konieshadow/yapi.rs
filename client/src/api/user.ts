import { AuthUserInfo } from '../types/AuthUserInfo';
import { UserInfo } from '../types/UserInfo';
import { UserLogin } from '../types/UserLogin';
import { UserReg } from '../types/UserReg';
import { getInstance } from './index';

export async function userStatus(): Promise<UserInfo> {
  return getInstance().get('/api/user/status');
}

export async function login(query: UserLogin): Promise<AuthUserInfo> {
  return getInstance().post('/api/user/login', query);
}

export async function reg(query: UserReg): Promise<AuthUserInfo> {
  return getInstance().post('/api/user/reg', query);
}
