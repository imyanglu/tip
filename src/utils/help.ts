export const formatName = (name: string) => {
  const arr = name.split("\\");
  return arr[arr.length - 1];
};

export function formatBytes(kb: number) {
  if (kb >= 1024 * 1024) {
    return (kb / (1024 * 1024)).toFixed(2) + " GB";
  } else if (kb >= 1024) {
    return (kb / 1024).toFixed(2) + " MB";
  } else {
    return kb + " KB";
  }
}
