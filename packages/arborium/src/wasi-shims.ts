/**
 * Minimal WASI shims for browser environment.
 * These provide stub implementations for WASI interfaces that
 * the grammar plugins require but don't actually use.
 */

// Error type for WASI I/O
class WasiError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'WasiError';
  }
}

// Minimal stream implementation
class OutputStream {
  write(_contents: Uint8Array): bigint {
    // Silently discard output
    return BigInt(0);
  }

  blockingWriteAndFlush(_contents: Uint8Array): void {
    // No-op
  }

  blockingFlush(): void {
    // No-op
  }

  checkWrite(): bigint {
    return BigInt(1024 * 1024); // Allow large writes
  }

  subscribe(): void {
    // No-op
  }
}

class InputStream {
  read(_len: bigint): Uint8Array {
    return new Uint8Array(0);
  }

  blockingRead(_len: bigint): Uint8Array {
    return new Uint8Array(0);
  }

  subscribe(): void {
    // No-op
  }
}

// Create the WASI import object expected by jco-generated modules
export function createWasiImports() {
  const stdout = new OutputStream();
  const stderr = new OutputStream();
  const stdin = new InputStream();

  // WASI interface implementations
  const environment = {
    getEnvironment: (): Array<[string, string]> => [],
    getArguments: (): string[] => [],
  };

  const exit = {
    exit: (status: { tag: string; val?: number }): void => {
      if (status.tag === 'err') {
        throw new WasiError(`WASI exit with error: ${status.val}`);
      }
    },
  };

  const stdinIface = { getStdin: () => stdin };
  const stdoutIface = { getStdout: () => stdout };
  const stderrIface = { getStderr: () => stderr };

  const wallClock = {
    now: (): { seconds: bigint; nanoseconds: number } => {
      const ms = Date.now();
      return {
        seconds: BigInt(Math.floor(ms / 1000)),
        nanoseconds: (ms % 1000) * 1_000_000,
      };
    },
    resolution: (): { seconds: bigint; nanoseconds: number } => {
      return { seconds: BigInt(0), nanoseconds: 1_000_000 };
    },
  };

  const filesystemTypes = {
    Descriptor: class {},
    DirectoryEntryStream: class {},
    filesystemErrorCode: () => null,
  };

  const preopens = {
    getDirectories: (): Array<[unknown, string]> => [],
  };

  const ioError = { Error: WasiError };
  const streams = { InputStream, OutputStream };

  const random = {
    getRandomBytes: (len: bigint): Uint8Array => {
      const bytes = new Uint8Array(Number(len));
      crypto.getRandomValues(bytes);
      return bytes;
    },
    getRandomU64: (): bigint => {
      const bytes = new Uint8Array(8);
      crypto.getRandomValues(bytes);
      const view = new DataView(bytes.buffer);
      return view.getBigUint64(0, true);
    },
  };

  // Return both versioned (@0.2.3) and unversioned imports for compatibility
  return {
    // Unversioned (used by published grammars)
    'wasi:cli/environment': environment,
    'wasi:cli/exit': exit,
    'wasi:cli/stdin': stdinIface,
    'wasi:cli/stdout': stdoutIface,
    'wasi:cli/stderr': stderrIface,
    'wasi:clocks/wall-clock': wallClock,
    'wasi:filesystem/types': filesystemTypes,
    'wasi:filesystem/preopens': preopens,
    'wasi:io/error': ioError,
    'wasi:io/streams': streams,
    'wasi:random/random': random,

    // Versioned @0.2.3 (for newer builds)
    'wasi:cli/environment@0.2.3': environment,
    'wasi:cli/exit@0.2.3': exit,
    'wasi:cli/stdin@0.2.3': stdinIface,
    'wasi:cli/stdout@0.2.3': stdoutIface,
    'wasi:cli/stderr@0.2.3': stderrIface,
    'wasi:clocks/wall-clock@0.2.3': wallClock,
    'wasi:filesystem/types@0.2.3': filesystemTypes,
    'wasi:filesystem/preopens@0.2.3': preopens,
    'wasi:io/error@0.2.3': ioError,
    'wasi:io/streams@0.2.3': streams,
    'wasi:random/random@0.2.3': random,
  };
}

// Grammar types import (the plugin exports these)
export const grammarTypesImport = {
  'arborium:grammar/types@0.1.0': {
    // Types are just interfaces, nothing to export
  },
};
