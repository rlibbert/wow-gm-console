import { invoke } from '@tauri-apps/api/core';
import type {
	AppError,
	ConnectionTestResult,
	DbConnectionConfig,
	DbConnectionTestResult,
	ItemFilter,
	ItemSummary,
	ServerProfile,
	Teleport
} from './types';

/** Thin, typed wrapper around every Tauri IPC call this app makes. Every
 * view should import from here rather than calling `invoke()` directly, so
 * the IPC contract has one place to update if a command signature changes.
 */

export function listProfiles(): Promise<ServerProfile[]> {
	return invoke('list_profiles');
}

export function addProfile(params: {
	name: string;
	host: string;
	soapPort: number;
	username: string;
	password: string;
	db?: DbConnectionConfig | null;
	dbPassword?: string | null;
}): Promise<ServerProfile> {
	return invoke('add_profile', { ...params, db: params.db ?? null, dbPassword: params.dbPassword ?? null });
}

export function updateProfile(params: {
	id: string;
	name: string;
	host: string;
	soapPort: number;
	username: string;
	password?: string;
	db?: DbConnectionConfig | null;
	dbPassword?: string | null;
}): Promise<ServerProfile> {
	return invoke('update_profile', {
		...params,
		password: params.password ?? null,
		db: params.db ?? null,
		dbPassword: params.dbPassword ?? null
	});
}

export function removeProfile(id: string): Promise<void> {
	return invoke('remove_profile', { id });
}

export function testConnection(id: string): Promise<ConnectionTestResult> {
	return invoke('test_connection', { id });
}

export function sendRawCommand(profileId: string, command: string): Promise<string> {
	return invoke('send_raw_command', { profileId, command });
}

export function gmRevive(profileId: string, target?: string): Promise<string> {
	return invoke('gm_revive', { profileId, target: target || null });
}

export function gmAddItem(profileId: string, item: string, count: number): Promise<string> {
	return invoke('gm_add_item', { profileId, item, count });
}

export function gmSetLevel(profileId: string, target: string, level: number): Promise<string> {
	return invoke('gm_set_level', { profileId, target, level });
}

export function gmSetGold(profileId: string, amountCopper: number): Promise<string> {
	return invoke('gm_set_gold', { profileId, amountCopper });
}

export function gmTeleportNamed(
	profileId: string,
	target: string,
	location: string
): Promise<string> {
	return invoke('gm_teleport_named', { profileId, target, location });
}

export function gmTeleportCoords(
	profileId: string,
	map: number,
	x: number,
	y: number,
	z: number
): Promise<string> {
	return invoke('gm_teleport_coords', { profileId, map, x, y, z });
}

export function gmKick(profileId: string, target: string, reason?: string): Promise<string> {
	return invoke('gm_kick', { profileId, target, reason: reason || null });
}

export function gmBanAccount(
	profileId: string,
	target: string,
	duration: string,
	reason: string
): Promise<string> {
	return invoke('gm_ban_account', { profileId, target, duration, reason });
}

export function gmServerInfo(profileId: string): Promise<string> {
	return invoke('gm_server_info', { profileId });
}

export function gmReloadTable(profileId: string, table: string): Promise<string> {
	return invoke('gm_reload_table', { profileId, table });
}

export function testDbConnection(id: string): Promise<DbConnectionTestResult> {
	return invoke('test_db_connection', { id });
}

export function listTeleports(profileId: string): Promise<Teleport[]> {
	return invoke('list_teleports', { id: profileId });
}

export function searchItems(
	profileId: string,
	filter: ItemFilter,
	limit?: number
): Promise<ItemSummary[]> {
	return invoke('search_items', { id: profileId, filter, limit: limit ?? null });
}

/** Type guard for the structured errors our Rust commands reject with. */
export function isAppError(e: unknown): e is AppError {
	return (
		typeof e === 'object' &&
		e !== null &&
		'kind' in e &&
		'message' in e
	);
}
