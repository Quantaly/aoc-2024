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
import { ExitMessage, InitMessage, OutputMessage } from "./communication";

const programs = {
  "01": wasm01,
  "02": wasm02,
  "03": wasm03,
  "04": wasm04,
} as const;

addEventListener(
  "message",
  // eslint-disable-next-line @typescript-eslint/no-misused-promises
  async ({ data: { program, input } }: MessageEvent<InitMessage>) => {
    if (program in programs) {
      const args = [program, "input.txt"];
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
            cwd.set("input.txt", new File(new TextEncoder().encode(input)));
            return cwd;
          })(),
        ),
      ];
      const wasi = new WASI(args, env, fds);
      const code = wasi.start(
        (await programs[program as keyof typeof programs]({
          wasi_snapshot_preview1: wasi.wasiImport,
        })) as Parameters<WASI["start"]>[0],
      );
      postMessage({ code } as ExitMessage);
    }
  },
  { once: true },
);
