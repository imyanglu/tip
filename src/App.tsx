import { Children, useEffect, useMemo, useRef, useState } from "react";

import "./App.css";
import { Process, SysInfo } from "../type";

import { invoke } from "@tauri-apps/api/core";
import { Button, Collapse } from "antd";

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
    const arr = name.split("\\");
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
  const killProcess = (pid: number) => {
    invoke("kill_process", { pid }).then(() => {
      initProcessInfo();
    });
  };
  const process = useMemo(() => {
    return Array.from(processMap.values())
      .map((item) => {
        return {
          key: item[0].name,
          sort: formatName(item[0].name),
          label: (
            <div className="flex items-center gap-x-2">
              <div className="w-[200px]"> {formatName(item[0].name)}</div>
              <div>
                虚拟:
                {formatBytes(item.reduce((s, i) => s + i.privateMemoryKb, 0))}
              </div>
            </div>
          ),
          children: (
            <div className="flex flex-col gap-y-2">
              {item.map((i) => (
                <div key={i.pid} className="flex items-center">
                  <div className="w-[100px] text-[13px]">PID: {i.pid}</div>
                  <div className="w-[150px]">
                    内存:{formatBytes(i.privateMemoryKb)}
                  </div>
                  <div className="w-[150px]">
                    物理内存:{formatBytes(i.memoryKb)}
                  </div>
                  <div className="flex-1 line-clamp-1">{i.name}</div>
                  <Button
                    size="small"
                    onClick={() => {
                      killProcess(i.pid);
                    }}
                  >
                    终止
                  </Button>
                </div>
              ))}
            </div>
          ),
        };
      })
      .sort((a, b) => a.sort.localeCompare(b.sort));
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
