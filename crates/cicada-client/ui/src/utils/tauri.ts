import { invoke as tauriInvoke } from "@tauri-apps/api/core";

/**
 * 检测当前是否运行在 Tauri 环境中
 */
export function isTauri(): boolean {
  return (
    typeof window !== "undefined" &&
    typeof (window as any).__TAURI_INTERNALS__ !== "undefined"
  );
}

/**
 * 安全地调用 Tauri 命令
 * 如果不在 Tauri 环境中，返回降级值或抛出错误
 */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    console.warn(`[Tauri] 命令 "${cmd}" 被跳过：当前不在 Tauri 环境中`);
    throw new Error("NOT_TAURI_ENV");
  }
  return tauriInvoke<T>(cmd, args);
}

/**
 * 尝试调用 Tauri 命令，失败时返回默认值
 */
export async function invokeOrDefault<T>(
  cmd: string,
  defaultValue: T,
  args?: Record<string, unknown>
): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (e) {
    if ((e as Error).message === "NOT_TAURI_ENV") {
      return defaultValue;
    }
    throw e;
  }
}
