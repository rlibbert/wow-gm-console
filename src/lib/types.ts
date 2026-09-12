export interface ServerProfile {
	id: string;
	name: string;
	host: string;
	soapPort: number;
	username: string;
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
	| 'storeError';

export interface AppError {
	kind: ErrorKind;
	message: string;
}

export interface ConnectionTestResult {
	success: boolean;
	latencyMs: number;
	message: string;
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
	}
}
