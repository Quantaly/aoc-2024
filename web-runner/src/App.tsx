import { useEffect, useRef, useState } from "react";
import { ExitMessage, InitMessage, OutputMessage } from "./communication";
import { formatSpace, formatTime } from "./formatting";

export default function App() {
  const [isRunning, setIsRunning] = useState(false);
  const [program, setProgram] = useState("01");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState<OutputMessage[]>([]);
  const [exitStats, setExitStats] = useState<ExitMessage | "Terminated">();

  const [boardSize, setBoardSize] = useState("standard");

  const stopRef = useRef<() => void>();

  function run() {
    stopRef.current?.();
    setIsRunning(true);
    setOutput([]);
    setExitStats(undefined);

    const worker = new Worker(new URL("./worker", import.meta.url), {
      type: "module",
    });
    const listener = ({ data }: MessageEvent<OutputMessage | ExitMessage>) => {
      if ("exitCode" in data) {
        setExitStats(data);
        setIsRunning(false);
      } else {
        setOutput((o) => [...o, data]);
      }
    };
    worker.addEventListener("message", listener);
    worker.postMessage({
      program,
      input,
      extraArgs:
        program === "14"
          ? boardSize === "example"
            ? ["11", "7"]
            : ["101", "103"]
          : program === "18"
            ? boardSize === "example"
              ? ["6", "12"]
              : ["70", "1024"]
            : [],
    } satisfies InitMessage);

    stopRef.current = () => {
      worker.removeEventListener("message", listener);
      worker.terminate();
    };
  }

  function stop() {
    setIsRunning(false);
    setExitStats("Terminated");
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
            <option value="12">12</option>
            <option value="13">13</option>
            <option value="14">14</option>
            <option value="15">15</option>
            <option value="16">16</option>
            <option value="17">17</option>
            <option value="18">18</option>
          </select>
        </label>
      </p>
      {program === "14" && (
        <>
          <p className="warning">
            Warning: My implementation for day 14 part 2 is to output all of the
            images until it finds a cycle, at which point I can Ctrl-F through
            the output for a bunch of asterisks in a row.{" "}
            <strong>
              This is far more than enough output to crash a browser tab.
            </strong>{" "}
            If you want to use this for part 1, be sure to hit the Stop button
            pretty quickly after hitting the Run button. At some point I might
            revisit this and try to make a more browser-friendly implementation
            but today I can’t be bothered, this was really dumb. I’m already
            annoyed that I have to have the extra room size dropdown.
          </p>
          <p>
            <label>
              Room size:{" "}
              <select
                value={boardSize}
                onInput={({ currentTarget }) => {
                  setBoardSize(currentTarget.value);
                }}
              >
                <option value="standard">Standard (101 x 103)</option>
                <option value="example">Example (11 x 7)</option>
              </select>
            </label>
          </p>
        </>
      )}
      {program === "18" && (
        <p>
          <label>
            Memory size:{" "}
            <select
              value={boardSize}
              onInput={({ currentTarget }) => {
                setBoardSize(currentTarget.value);
              }}
            >
              <option value="standard">
                Standard (70 x 70, first 1024 for part 1)
              </option>
              <option value="example">
                Example (6 x 6, first 12 for part 1)
              </option>
            </select>
          </label>
        </p>
      )}
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
      <ul className="output">
        {output.map(({ line, stream }, i) => (
          <li key={i} className={stream}>
            {line}
          </li>
        ))}
      </ul>
      {exitStats === "Terminated" ? (
        <p className="stderr">Terminated</p>
      ) : (
        exitStats && (
          <ul>
            <li>Exit code: {exitStats.exitCode}</li>
            <li>Execution time: {formatTime(exitStats.duration)}</li>
            <li>Memory usage: {formatSpace(exitStats.memorySize)}</li>
          </ul>
        )
      )}
    </>
  );
}
