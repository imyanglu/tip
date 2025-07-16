import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useState } from "react";
import { Process } from "../../../type";
import { directoryClass, formatBytes, formatName } from "../../utils/help";
import { Button, Collapse, Input } from "antd";

export default function () {
  const [config, setConfig] = useState({ duration: 1 });
  const getProcessInfo = async () => {
    await invoke("watch_process");
  };
  const [processMap, setProcessMap] = useState<Map<string, Process[]>>(
    new Map()
  );

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
              <div className="w-[300px] break-all line-clamp-1 flex items-center gap-x-[12px]">
                {formatName(item[0].name)}
                {directoryClass(item[0].name) === "system" ? (
                  <div className="w-[6px] aspect-square rounded-full bg-[#1CD66C]"></div>
                ) : (
                  <></>
                )}
              </div>
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
                  <div className="flex-1 line-clamp-1" title={i.name}>
                    {i.name}
                  </div>
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
  const listenProcess = async () => {
    listen("process_change", (e) => {
      const payload = e.payload;
      combineProcess(payload as Process[]);
    });
  };
  useEffect(() => {
    getProcessInfo();
    listenProcess();
  }, []);
  return (
    <div>
      <Input prefix="扫描间隔"  />
      <Collapse items={process} />
    </div>
  );
}
