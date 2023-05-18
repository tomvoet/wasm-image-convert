import init, { convert_image } from "@/wasm/pkg";

self.addEventListener(
  "message",
  async function (
    e: MessageEvent<{
      inputFile: Uint8Array;
      inputType: string;
      outputType: string;
    }>
  ) {
    await init();

    const res = convert_image(
      e.data.inputFile,
      e.data.inputType,
      e.data.outputType
    );

    self.postMessage(res);
  },
  false
);
