export interface DbConnectionConfig {
	host: string;
	port: number;
	database: string;
	username: string;
}

export interface ServerProfile {
	id: string;
	name: string;
	host: string;
	soapPort: number;
	username: string;
	db?: DbConnectionConfig | null;
}

export type ErrorKind =
	| 'authFailed'
	| 'insufficientPrivilege'
	| 'networkError'
	| 'timeout'
	| 'commandFailed'
	| 'malformedResponse'
	| 'notFound'
	| 'keychainError'
	| 'storeError'
	| 'databaseError';

export interface AppError {
	kind: ErrorKind;
	message: string;
}

export interface ConnectionTestResult {
	success: boolean;
	latencyMs: number;
	message: string;
}

export interface DbConnectionTestResult {
	success: boolean;
	latencyMs: number;
	message: string;
}

export interface Teleport {
	id: number;
	name: string;
	map: number;
	positionX: number;
	positionY: number;
	positionZ: number;
	orientation: number;
}

export interface ItemSummary {
	entry: number;
	name: string;
	class: number;
	subclass: number;
	quality: number;
	requiredLevel: number;
	itemLevel: number;
	inventoryType: number;
}

export interface ItemFilter {
	nameSubstring?: string;
	class?: number;
	subclass?: number;
	qualityMin?: number;
	requiredLevelMax?: number;
}

export interface LogEntry {
	id: number;
	timestamp: Date;
	source: 'console' | 'action';
	command: string;
	result?: string;
	error?: AppError;
}

export function errorKindLabel(kind: ErrorKind): string {
	switch (kind) {
		case 'authFailed':
			return 'Authentication failed — check username/password';
		case 'insufficientPrivilege':
			return 'Account needs GM level 3 (Administrator) or higher';
		case 'networkError':
			return 'Network error — check host/port and that SOAP is enabled';
		case 'timeout':
			return 'Request timed out';
		case 'commandFailed':
			return 'Command failed';
		case 'malformedResponse':
			return 'Server returned an unexpected response';
		case 'notFound':
			return 'Profile not found';
		case 'keychainError':
			return 'Could not access the system keychain';
		case 'storeError':
			return 'Could not read/write saved profiles';
		case 'databaseError':
			return 'Database error — check DB host/port/credentials and that the read-only MySQL user has SELECT privileges on the world database';
	}
}
