import type { LogEntry } from './types';

let nextId = 1;

class LogStore {
	entries = $state<LogEntry[]>([]);

	add(entry: Omit<LogEntry, 'id' | 'timestamp'>) {
		this.entries.push({ ...entry, id: nextId++, timestamp: new Date() });
	}

	clear() {
		this.entries = [];
	}
}

export const log = new LogStore();
