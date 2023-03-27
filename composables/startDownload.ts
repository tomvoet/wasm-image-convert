export default function startDownload(file: Uint8Array, filename: string) {
  console.log("startDownload");

  const blob = new Blob([file], { type: "application/octet-stream" });

  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  a.remove();
  URL.revokeObjectURL(url);

  console.log("startDownload done");
}
