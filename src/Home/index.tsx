import React from 'react';
import { Card } from 'antd';

const Home: React.FC = () => {
  return (
    <div>
      <Card title="软件信息" style={{ width: 300 }}>
        <p>操作系统：Windows 10</p>
        <p>浏览器：Google Chrome</p>
        <p>办公软件：Microsoft Office</p>
        <p>图形设计工具：Adobe Photoshop</p>
      </Card>
    </div>
  );
};

export default Home;
