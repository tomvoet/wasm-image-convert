import init, { convert_image } from "@/wasm/pkg";
import { WorkerRequest, WorkerResponse } from "./convert.d";

self.addEventListener(
  "message",
  async function (e: MessageEvent<WorkerRequest>) {
    await init();

    //const res = convert_image(
    //  e.data.inputFile,
    //  e.data.inputType,
    //  e.data.outputType
    //)
    //
    //console.log("b");
    //
    //self.postMessage(res);

    try {
      const res = convert_image(
        e.data.inputFile,
        e.data.inputType,
        e.data.outputType
      );

      let response: WorkerResponse = {
        success: true,
        data: res,
      };

      self.postMessage(response);
    } catch (e) {
      let response: WorkerResponse = {
        success: false,
        error: String(e),
      };

      self.postMessage(response);
    }
  },
  false
);
