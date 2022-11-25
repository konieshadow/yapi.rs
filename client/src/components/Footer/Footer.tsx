import './Footer.scss';
import { Col, Row } from 'antd';
import { FC } from 'react';
import IconFont from '../IconFont';

const version = '0.1.0';

interface FooterProps {
  footList: FootItemProps[];
}

const Footer: FC<FooterProps> = (props) => (
  <div className="footer-wrapper">
    <Row className="footer-container">
      {props.footList.map(function (item, i) {
        return (
          <FootItem
            key={i}
            linkList={item.linkList}
            title={item.title}
            iconType={item.iconType}
          />
        );
      })}
    </Row>
  </div>
);

interface FootItemProps {
  linkList: {
    itemLink: string;
    itemTitle: string;
  }[];
  title: string
  iconType?: string;
}

const FootItem: FC<FootItemProps> = (props) => (
  <Col span={6}>
    <h4 className="title">
      {props.iconType ? <IconFont type={props.iconType} className="icon" /> : ''}
      {props.title}
    </h4>
    {props.linkList.map(function (item, i) {
      return (
        <p key={i}>
          <a href={item.itemLink} className="link">
            {item.itemTitle}
          </a>
        </p>
      );
    })}
  </Col>
);

export const defaultFootList: FootItemProps[] = [
  {
    title: 'GitHub',
    iconType: 'github',
    linkList: [
      {
        itemTitle: 'YApi Pro 源码仓库',
        itemLink: 'https://github.com/yapi-pro/yapi'
      }
    ]
  },
  {
    title: '团队',
    iconType: 'team',
    linkList: [
      {
        itemTitle: 'YApi-Pro',
        itemLink: 'https://github.com/yapi-pro'
      },
      {
        itemTitle: 'YMFE',
        itemLink: 'https://ymfe.org/'
      }
    ]
  },
  {
    title: '反馈',
    iconType: 'aliwangwang-o',
    linkList: [
      {
        itemTitle: 'Github Issues',
        itemLink: 'https://github.com/yapi-pro/yapi/issues'
      },
      {
        itemTitle: 'Github Pull Requests',
        itemLink: 'https://github.com/yapi-pro/yapi/pulls'
      }
    ]
  },
  {
    title: ' ',
    linkList: [
      {
        itemTitle: `版本: ${version} `,
        itemLink: 'https://github.com/yapi-pro/yapi/blob/master/CHANGELOG.md'
      },
      {
        itemTitle: '使用文档',
        itemLink: 'https://hellosean1025.github.io/yapi/'
      }
    ]
  }
];

export default Footer;
