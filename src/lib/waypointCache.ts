import type { Teleport } from './types';
import * as api from './tauriApi';

// The ~2000-row game_tele table barely changes, so fetch it once per
// profile and reuse across picker opens within the same app session,
// rather than refetching every time the picker is toggled open.
const cache = new Map<string, Teleport[]>();

export async function getTeleports(profileId: string, forceRefresh = false): Promise<Teleport[]> {
	if (!forceRefresh && cache.has(profileId)) {
		return cache.get(profileId)!;
	}
	const teleports = await api.listTeleports(profileId);
	cache.set(profileId, teleports);
	return teleports;
}

export function invalidate(profileId: string) {
	cache.delete(profileId);
}
