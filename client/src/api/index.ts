import { message } from 'antd';
import axios, { Axios } from 'axios';
import { AuthUserInfo } from '../types/AuthUserInfo';

const JWT_TOKEN_STORAGE_KEY = "_yapi_token";
const JWT_SCHEMA_PREFIX = "Token ";

let _cachedToken: string | null;
let _instance: Axios | undefined;

interface ResData<T> {
  errcode: number,
  errmsg: string,
  data?: T,
}

export class ApiError extends Error {
  constructor(public errcode: number, public errmsg: string) {
    super('api error ' + errcode + ': ' + errmsg);
  }
}

export class UnauthorizeError extends Error {
  constructor() {
    super("User is unauthorized");
  }
}

export class PermissionDenyError extends Error {
  constructor() {
    super("Permission denied");
  }
}

function getToken(): string | null {
  if (_cachedToken == null) {
    _cachedToken = window.localStorage.getItem(JWT_TOKEN_STORAGE_KEY);
  }
  return _cachedToken;
}

function setToken(value: string) {
  _cachedToken = value;
  window.localStorage.setItem(JWT_TOKEN_STORAGE_KEY, value);
}

function removeToken() {
  _cachedToken = '';
  window.localStorage.removeItem(JWT_TOKEN_STORAGE_KEY);
}

export function getInstance(): Axios {
  if (_instance == undefined) {
    _instance = axios.create({
      baseURL: '',
      timeout: 15000,
    });

    _instance.interceptors.request.use(config => {
      // 添加认证请求头
      const token = getToken();
      if (token != null && token !== '') {
        config.headers = {
          ...config.headers,
          'Authorization': JWT_SCHEMA_PREFIX + token,
        };
      }

      return config;
    });

    _instance.interceptors.response.use(response => {
      if (response.status != 200) {
        // 请求错误
        message.error('Network Error');
        throw new Error('request error with code ' + response.status);
      }

      if (!response.data) {
        message.error('Network Error');
        throw new Error('reponse data format invalid');
      }

      const resData = response.data as ResData<any>;
      if (resData.errcode !== 0) {

        if (resData.errcode == 40011) {
          // 登录失效
          removeToken();
          throw new UnauthorizeError();
        }

        if (resData.errcode == 40013) {
          // 权限不足
          throw new PermissionDenyError();
        }

        message.error(resData.errmsg);
        throw new ApiError(resData.errcode, resData.errmsg);
      }

      const requestUrl = response.request?.responseURL ?? '';
      if (requestUrl.indexOf('/user/login') >= 0 || requestUrl.indexOf('/user/reg') >= 0) {
        // 处理登录token
        const userInfo = resData.data as AuthUserInfo;
        setToken(userInfo.token);
      }

      return resData.data;
    });
  }
  return _instance;
}

export function handleLogout() {
  removeToken();
}
