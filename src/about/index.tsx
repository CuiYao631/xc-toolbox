import React, { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import '../styles/about.css';

interface SystemInfo {
  os: string; // 假设 "os" 属性的类型是字符串
  memory_total: string;
}


const About: React.FC = () => {
  const [os, setOs] = useState("");
  const [memory_total, setMemory_total] = useState("");
  const myFunction = async () => {

    try {
      const result = await invoke<SystemInfo>("system_info"); // 使用类型断言

      // 在页面加载完成时执行的代码
      console.log('Component has loaded.', result);

      if (result && result.os) {
        // 更新操作系统状态
        setOs(result.os);
      }
      if (result && result.memory_total) {
        // 更新内存状态
        setMemory_total(result.memory_total);
      }
    } catch (error) {
      console.error('Error retrieving system information:', error);
    }
  };
  useEffect(() => {
    myFunction();
  }, []);

  return (
      <div className="container">
        <div className="software-info text-center">
          <h2>软件信息</h2>
          <p><strong>软件名称:</strong> 我的软件</p>
          <p><strong>版本:</strong> 1.0.0</p>
          <p><strong>开发者:</strong> 您的姓名或公司</p>
          <p><strong>联系信息:</strong> 您的联系信息</p>
          <p><strong>描述:</strong> 这是一个示例软件，用于演示关于页面。</p>
        </div>
        <div className="system-info text-center">
          <h2>系统信息</h2>
        <p><strong>操作系统:</strong>{os}</p>
          <p><strong>处理器:</strong> 未知</p>
        <p><strong>内存:</strong> {memory_total} GB RAM</p>
          <p><strong>存储:</strong> 512 GB SSD</p>
        </div>
      </div>
  );
}

export default About;
