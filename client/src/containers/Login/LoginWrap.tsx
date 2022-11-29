import './Login.scss';
import { FC } from 'react';
import { Tabs } from 'antd';
import TabPane from 'antd/es/tabs/TabPane';
import LoginForm from './login';
import RegForm from './Reg';

const tabItems = [
  { label: '登录', key: '1', children: <LoginForm /> },
  { label: '注册', key: '2', children: <RegForm /> }
]

const LoginWrapper: FC = () => (
  <Tabs defaultActiveKey={""}
    className="login-form"
    tabBarStyle={{ border: 'none' }}
    items={tabItems} />
);

export default LoginWrapper;
