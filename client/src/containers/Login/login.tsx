import { useRequest } from 'ahooks';
import { Button, Form, Input, message, Radio } from 'antd';
import { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import { login } from '../../api/user';
import IconFont from '../../components/IconFont';

const FormItem = Form.Item;

const formItemStyle = {
  marginBottom: '.16rem'
};

const changeHeight = {
  height: '.42rem'
};

const LoginForm: FC = () => {
  const [form] = Form.useForm();
  const navigate = useNavigate();

  const { run: runLogin } = useRequest(login, {
    manual: true,
    onSuccess: () => {
      navigate("/group", { replace: true });
    }
  });

  const onFinish = (values: { email: string, password: string }) => {
    runLogin({ email: values.email, password: values.password });
  };

  return (
    <Form form={form} onFinish={onFinish}>
      <FormItem style={formItemStyle}
        name="email"
        rules={[{ required: true, message: '请输入用户名!' }, { min: 2, max: 30, message: '用户名格式不正确!' }]}>
        <Input style={changeHeight}
          prefix={<IconFont type="mail" style={{ fontSize: 13 }} />}
          placeholder="Email" />
      </FormItem>

      <FormItem style={formItemStyle}
        name="password"
        rules={[{ required: true, message: '请输入密码!' }]}>
        <Input style={changeHeight}
          prefix={<IconFont type="lock" style={{ fontSize: 13 }} />}
          type="password"
          placeholder="Password" />
      </FormItem>

      <FormItem style={formItemStyle}>
        <Button style={changeHeight}
          type="primary"
          htmlType="submit"
          className="login-form-button">
          登录
        </Button>
      </FormItem>
    </Form>
  )
};

export default LoginForm;
