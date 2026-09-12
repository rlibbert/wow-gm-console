import type { ServerProfile } from './types';

class AppState {
	profiles = $state<ServerProfile[]>([]);
	activeProfileId = $state<string | null>(null);
	view = $state<'connections' | 'dashboard' | 'console'>('connections');

	get activeProfile(): ServerProfile | null {
		return this.profiles.find((p) => p.id === this.activeProfileId) ?? null;
	}
}

export const appState = new AppState();
