import { useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { SysInfo } from "../type";

import { invoke } from "@tauri-apps/api/core";

function formatBytes(kb: number) {
  if (kb >= 1024 * 1024) {
    return (kb / (1024 * 1024)).toFixed(2) + " GB";
  } else if (kb >= 1024) {
    return (kb / 1024).toFixed(2) + " MB";
  } else {
    return kb + " KB";
  }
}
function App() {
  const [systemInfo, setSystemInfo] = useState<SysInfo | null>(null);
  const initSystemInfo = async () => {
    // 调用get_system_info函数，获取系统信息
    // 打印系统信息
    const res = await invoke("get_system_info");
    console.log(res);
    // 设置系统信息
    setSystemInfo(res as SysInfo);
  };
  // useEffect函数，在组件加载时执行
  useEffect(() => {
    initSystemInfo();
  }, []);

  if (!systemInfo) return null;
  return (
    <main className="container text-[13px]">
      <p>名称:{systemInfo.hostname}</p>
      <p>
        类型:{systemInfo.osType}&nbsp;版本:{systemInfo.osRelease}
      </p>
      <p>
        CPU:{systemInfo.cpuNum}&nbsp;转数:{systemInfo.cpuSpeed}MHZ&nbsp;
      </p>
      <p>
        当前运行进程数:
        {systemInfo.procTotal}
      </p>
      <p>
        磁盘容量:{formatBytes(systemInfo.diskTotal)}&nbsp;空闲:
        {formatBytes(systemInfo.diskFree)}
      </p>
      <p>
        内存:{formatBytes(systemInfo.memInfo.total)} &nbsp;空闲:
        {formatBytes(systemInfo.memInfo.free)}
        &nbsp; 缓存:{formatBytes(systemInfo.memInfo.cached)}
        &nbsp; 总交换空间:{formatBytes(systemInfo.memInfo.swapTotal)}
        &nbsp; 空闲交换空间:{formatBytes(systemInfo.memInfo.swapFree)}
      </p>
      <p className="mt-[12px]">
        临时目录大小:{formatBytes(systemInfo.tempDirSize / 1024)}
        <button className="ml-[12px]">清除</button>
      </p>
    </main>
  );
}

export default App;
