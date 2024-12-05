export interface InitMessage {
  readonly program: string;
  readonly input: string;
}

export interface OutputMessage {
  readonly line: string;
  readonly stream: "stdout" | "stderr";
}

export interface ExitMessage {
  readonly code: number;
}
