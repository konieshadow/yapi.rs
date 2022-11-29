import { useMount } from 'ahooks';
import { Card, Col, Row } from 'antd';
import { FC } from 'react';
import { handleLogout } from '../../api';
import LogoSVG from '../../components/LogoSVG';
import LoginWrapper from './LoginWrap';

const LoginContainer: FC = () => {
  useMount(() => handleLogout());

  return (
    <div className="g-body login-body">
      <div className="m-bg">
        <div className="m-bg-mask m-bg-mask0"></div>
        <div className="m-bg-mask m-bg-mask1"></div>
        <div className="m-bg-mask m-bg-mask2"></div>
        <div className="m-bg-mask m-bg-mask3"></div>
      </div>
      <div className="main-one login-container">
        <div className="container">
          <Row justify="center">
            <Col xs={20} sm={16} md={12} lg={8} className="container-login">
              <Card className="card-login">
                <h2 className="login-title">YApi Pro</h2>
                <div className="login-logo">
                  <LogoSVG length="100px" />
                </div>
                <LoginWrapper />
              </Card>
            </Col>
          </Row>
        </div>
      </div>
    </div>
  )
};

export default LoginContainer;
