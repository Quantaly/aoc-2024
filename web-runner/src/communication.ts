export interface InitMessage {
  readonly program: string;
  readonly input: string;
}

export interface OutputMessage {
  readonly line: string;
  readonly stream: "stdout" | "stderr";
}

export interface ExitMessage {
  readonly exitCode: number;
  readonly duration: number;
  readonly memorySize: number;
}
