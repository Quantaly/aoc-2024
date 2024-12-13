const secondsFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "second",
  unitDisplay: "short",
  minimumFractionDigits: 3,
  maximumFractionDigits: 3,
});
const millisecondsFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "millisecond",
  unitDisplay: "short",
  minimumFractionDigits: 3,
  maximumFractionDigits: 3,
});
const microsecondsFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "microsecond",
  unitDisplay: "short",
  minimumFractionDigits: 1,
  maximumFractionDigits: 3,
});
const nanosecondsFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "nanosecond",
  unitDisplay: "short",
  maximumFractionDigits: 0,
});

export function formatTime(milliseconds: number): string {
  if (milliseconds >= 1_000) {
    return secondsFormat.format(milliseconds / 1_000);
  } else if (milliseconds >= 1) {
    return millisecondsFormat.format(milliseconds / 1);
  } else if (milliseconds >= 0.001) {
    return microsecondsFormat.format(milliseconds * 1_000);
  } else {
    return nanosecondsFormat.format(milliseconds / 1_000_000);
  }
}

const gigabytesFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "gigabyte",
  unitDisplay: "short",
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});
const megabytesFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "megabyte",
  unitDisplay: "short",
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});
const kilobytesFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "kilobyte",
  unitDisplay: "short",
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});
const bytesFormat = new Intl.NumberFormat(undefined, {
  style: "unit",
  unit: "byte",
  unitDisplay: "short",
  maximumFractionDigits: 0,
});

export function formatSpace(bytes: number): string {
  if (bytes >= 1_000_000_000) {
    return gigabytesFormat.format(bytes / 1_000_000_000);
  } else if (bytes >= 1_000_000) {
    return megabytesFormat.format(bytes / 1_000_000);
  } else if (bytes >= 1_000) {
    return kilobytesFormat.format(bytes / 1_000);
  } else {
    // should never get here because one page of Wasm memory is 64 KiB
    return bytesFormat.format(bytes);
  }
}
