import { useEffect, useRef, useState } from "react";
import { ExitMessage, InitMessage, OutputMessage } from "./communication";

export default function App() {
  const [isRunning, setIsRunning] = useState(false);
  const [program, setProgram] = useState("01");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState<OutputMessage[]>([]);

  const stopRef = useRef<() => void>();

  function run() {
    stopRef.current?.();
    setIsRunning(true);
    setOutput([]);

    const worker = new Worker(new URL("./worker", import.meta.url), {
      type: "module",
    });
    const listener = ({ data }: MessageEvent<OutputMessage | ExitMessage>) => {
      if ("code" in data) {
        if (data.code === 0) {
          setOutput((o) => [...o, { line: "(Exit code 0)", stream: "stdout" }]);
        } else {
          setOutput((o) => [
            ...o,
            { line: `(Exit code ${String(data.code)})`, stream: "stderr" },
          ]);
        }
        setIsRunning(false);
      } else {
        setOutput((o) => [...o, data]);
      }
    };
    worker.addEventListener("message", listener);
    worker.postMessage({ program, input } satisfies InitMessage);

    stopRef.current = () => {
      worker.removeEventListener("message", listener);
      worker.terminate();
    };
  }

  function stop() {
    setIsRunning(false);
    setOutput((o) => [...o, { line: "Terminated", stream: "stderr" }]);
    stopRef.current?.();
  }

  useEffect(
    () => () => {
      stopRef.current?.();
    },
    [],
  );

  return (
    <>
      <h1>
        Kai’s <a href="https://adventofcode.com/2024">Advent of Code 2024</a>{" "}
        Solutions
      </h1>
      <p>
        <a href="https://github.com/Quantaly/aoc-2024">Source code</a>
      </p>
      <p>
        <label>
          Program:{" "}
          <select
            disabled={isRunning}
            value={program}
            onInput={({ currentTarget }) => {
              setProgram(currentTarget.value);
            }}
          >
            <option value="01">01</option>
            <option value="02">02</option>
            <option value="03">03</option>
            <option value="04">04</option>
            <option value="05">05</option>
            <option value="06">06</option>
            <option value="07">07</option>
            <option value="08">08</option>
            <option value="09">09</option>
            <option value="10">10</option>
            <option value="11">11</option>
          </select>
        </label>
      </p>
      <label>
        <p>Input:</p>
        <div>
          <textarea
            disabled={isRunning}
            value={input}
            onInput={({ currentTarget }) => {
              setInput(currentTarget.value);
            }}
          />
        </div>
      </label>
      <p>
        <button disabled={isRunning} onClick={run}>
          Run
        </button>
        <button disabled={!isRunning} onClick={stop}>
          Stop
        </button>
      </p>
      <ul>
        {output.map(({ line, stream }, i) => (
          <li key={i} className={stream}>
            {line}
          </li>
        ))}
      </ul>
    </>
  );
}
