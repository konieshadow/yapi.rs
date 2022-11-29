import { useRequest } from 'ahooks';
import { Button, Form, Input, Radio } from 'antd';
import { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import { reg } from '../../api/user';
import IconFont from '../../components/IconFont';

const FormItem = Form.Item;

const formItemStyle = {
  marginBottom: '.16rem'
};

const changeHeight = {
  height: '.42rem'
};

const RegForm: FC = () => {
  const [form] = Form.useForm();
  const navigate = useNavigate();

  const { run: runReg } = useRequest(reg, {
    manual: true,
    onSuccess: () => {
      navigate("/group", { replace: true });
    }
  });

  const onFinish = (values: { username: string, email: string, password: string, confirm: string }) => {
    runReg({ username: values.username, email: values.email, password: values.password });
  };

  const passworConfirmValidator = (_: unknown, value: string) => {
    if (!value || value === form.getFieldValue('password')) {
      return Promise.resolve();
    } else {
      return Promise.reject(new Error('The two passwords that you entered do not match!'));
    }
  }

  return <Form form={form} onFinish={onFinish}>
    <FormItem style={formItemStyle}
      name="username"
      rules={[{ required: true, message: '请输入用户名!' }, { min: 2, max: 30, message: '用户名格式不正确!' }]}>
      <Input style={changeHeight}
        prefix={<IconFont type="user" style={{ fontSize: 13 }} />}
        placeholder="Username" />
    </FormItem>

    <FormItem style={formItemStyle}
      name="email"
      rules={[{ type: 'email', max: 50, required: true, message: '请输入正确的email!' }]}>
      <Input style={changeHeight}
        prefix={<IconFont type="mail" style={{ fontSize: 13 }} />}
        placeholder="Email" />
    </FormItem>

    <FormItem style={formItemStyle}
      name="password"
      rules={[{ required: true, message: '请输入密码!' }, { pattern: /[A-Za-z\d$@$!%*#?&]{8,20}/, message: '密码格式不正确!' }]}>
      <Input style={changeHeight}
        prefix={<IconFont type="lock" style={{ fontSize: 13 }} />}
        type="password"
        placeholder="Password" />
    </FormItem>

    <FormItem style={formItemStyle}
      name="confirm"
      dependencies={["password"]}
      rules={[{ required: true, message: '请再次输入密码!' }, { pattern: /[A-Za-z\d$@$!%*#?&]{8,20}/, message: '密码格式不正确!' }, { validator: passworConfirmValidator, message: '两次输入的密码不一致啊!' }]}>
      <Input style={changeHeight}
        prefix={<IconFont type="lock" style={{ fontSize: 13 }} />}
        type="password"
        placeholder="Confirm Password" />
    </FormItem>

    <FormItem style={formItemStyle}>
      <Button style={changeHeight}
        type="primary"
        htmlType="submit"
        className="login-form-button">
        注册
      </Button>
    </FormItem>
  </Form>
};

export default RegForm;
