export interface SysInfo {
  hostname: string;
  cpuNum: number;
  cpuSpeed: number;
  procTotal: number;
  osRelease: string;
  osType: string;
  diskTotal: number;
  diskFree: number;
  memInfo: MemInfo;
  tempDirSize: number;
}

export interface MemInfo {
  total: number;
  free: number;
  avail: number;
  buffers: number;
  cached: number;
  swapTotal: number;
  swapFree: number;
}

export type Process = {
  name: string;
  pid: number;
  path: string;
  memoryKb: number;
  privateMemoryKb: number;
};
