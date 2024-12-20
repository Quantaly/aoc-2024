import {
  ConsoleStdout,
  File,
  Inode,
  OpenFile,
  PreopenDirectory,
  WASI,
} from "@bjorn3/browser_wasi_shim";
import wasm01 from "../../target/wasm32-wasip1/release/01.wasm?init";
import wasm02 from "../../target/wasm32-wasip1/release/02.wasm?init";
import wasm03 from "../../target/wasm32-wasip1/release/03.wasm?init";
import wasm04 from "../../target/wasm32-wasip1/release/04.wasm?init";
import wasm05 from "../../target/wasm32-wasip1/release/05.wasm?init";
import wasm06 from "../../target/wasm32-wasip1/release/06.wasm?init";
import wasm07 from "../../target/wasm32-wasip1/release/07.wasm?init";
import wasm08 from "../../target/wasm32-wasip1/release/08.wasm?init";
import wasm09 from "../../target/wasm32-wasip1/release/09.wasm?init";
import wasm10 from "../../target/wasm32-wasip1/release/10.wasm?init";
import wasm11 from "../../target/wasm32-wasip1/release/11.wasm?init";
import wasm12 from "../../target/wasm32-wasip1/release/12.wasm?init";
import wasm13 from "../../target/wasm32-wasip1/release/13.wasm?init";
import wasm14 from "../../target/wasm32-wasip1/release/14.wasm?init";
import wasm15 from "../../target/wasm32-wasip1/release/15.wasm?init";
import wasm16 from "../../target/wasm32-wasip1/release/16.wasm?init";
import wasm17 from "../../target/wasm32-wasip1/release/17.wasm?init";
import wasm18 from "../../target/wasm32-wasip1/release/18.wasm?init";
import wasm19 from "../../target/wasm32-wasip1/release/19.wasm?init";
import wasm20 from "../../target/wasm32-wasip1/release/20.wasm?init";
import { ExitMessage, InitMessage, OutputMessage } from "./communication";

const programs = {
  "01": wasm01,
  "02": wasm02,
  "03": wasm03,
  "04": wasm04,
  "05": wasm05,
  "06": wasm06,
  "07": wasm07,
  "08": wasm08,
  "09": wasm09,
  "10": wasm10,
  "11": wasm11,
  "12": wasm12,
  "13": wasm13,
  "14": wasm14,
  "15": wasm15,
  "16": wasm16,
  "17": wasm17,
  "18": wasm18,
  "19": wasm19,
  "20": wasm20,
} as const;

addEventListener(
  "message",
  // eslint-disable-next-line @typescript-eslint/no-misused-promises
  async ({
    data: { program, input, extraArgs },
  }: MessageEvent<InitMessage>) => {
    if (program in programs) {
      const args = [program, "input.txt", ...extraArgs];
      const env: string[] = [];
      const fds = [
        // stdin
        new OpenFile(new File([], { readonly: true })),
        // stdout
        ConsoleStdout.lineBuffered((line) => {
          postMessage({ line, stream: "stdout" } satisfies OutputMessage);
        }),
        // stderr
        ConsoleStdout.lineBuffered((line) => {
          postMessage({ line, stream: "stderr" } satisfies OutputMessage);
        }),
        // preopens
        new PreopenDirectory(
          ".",
          (() => {
            const cwd = new Map<string, Inode>();
            cwd.set(
              "input.txt",
              new File(new TextEncoder().encode(addTrailingNewline(input))),
            );
            return cwd;
          })(),
        ),
      ];
      const wasi = new WASI(args, env, fds);
      const wasm = (await programs[program as keyof typeof programs]({
        wasi_snapshot_preview1: wasi.wasiImport,
      })) as WebAssembly.Instance & {
        exports: {
          memory: WebAssembly.Memory;
          _start: () => void;
        };
      };
      performance.mark("program start");
      const exitCode = wasi.start(wasm);
      performance.mark("program finish");
      const measurement = performance.measure(
        "program execution",
        "program start",
        "program finish",
      );
      postMessage({
        exitCode,
        duration: measurement.duration,
        memorySize: wasm.exports.memory.buffer.byteLength,
      } satisfies ExitMessage);
      close();
    }
  },
  { once: true },
);

function addTrailingNewline(content: string): string {
  if (!content.endsWith("\n")) {
    return content + "\n";
  } else {
    return content;
  }
}
