import { Children, useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { Process, SysInfo } from "../type";

import { invoke } from "@tauri-apps/api/core";
import { Collapse } from "antd";

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
  const [processMap, setProcessMap] = useState<Map<string, Process[]>>(
    new Map()
  );
  const initSystemInfo = async () => {
    // 调用get_system_info函数，获取系统信息
    // 打印系统信息
    const res = await invoke("get_system_info");

    // 设置系统信息
    setSystemInfo(res as SysInfo);
  };
  const formatName = (name: string) => {
    const arr = name.split("//");
    return arr[arr.length - 1];
  };
  const combineProcess = (arr: Process[]) => {
    const map = new Map<string, Process[]>();
    arr.forEach((item) => {
      if (map.has(item.name)) {
        map.get(item.name)?.push(item);
      } else {
        map.set(item.name, [item]);
      }
    });
    setProcessMap(map);
  };
  const process = useMemo(() => {
    return Array.from(processMap.values()).map((item) => {
      return {
        key: item[0].name,
        label: formatName(item[0].name),
        children: (
          <div>
            {item.map((i) => (
              <div key={i.pid}>
                {i.pid}&nbsp;&nbsp;&nbsp;
                {i.name}
                {i.memoryKb}
              </div>
            ))}
          </div>
        ),
      };
    });
  }, [processMap]);
  const initProcessInfo = async () => {
    const res: Process[] = await invoke("get_process_info");
    combineProcess(res);
  };
  // useEffect函数，在组件加载时执行
  useEffect(() => {
    initSystemInfo();
    setInterval(() => {
      initProcessInfo();
    }, 1000);
  }, []);

  if (!systemInfo) return null;
  return (
    <main className="container text-[13px]">
      <Collapse accordion items={process} />
    </main>
  );
}

export default App;
