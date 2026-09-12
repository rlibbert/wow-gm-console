import { log } from './log.svelte';
import { isAppError } from './tauriApi';
import type { AppError, LogEntry } from './types';

/** Runs a Tauri command that returns a raw result string, logging both the
 * attempted command text and its outcome to the shared console/action log
 * so curated buttons and the raw console share one audit trail.
 */
export async function runAction(
	source: LogEntry['source'],
	commandLabel: string,
	call: () => Promise<string>
): Promise<void> {
	try {
		const result = await call();
		log.add({ source, command: commandLabel, result });
	} catch (e) {
		const error: AppError = isAppError(e)
			? e
			: { kind: 'networkError', message: String(e) };
		log.add({ source, command: commandLabel, error });
	}
}
