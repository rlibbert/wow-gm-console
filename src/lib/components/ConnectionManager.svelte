<script lang="ts">
	import { appState } from '$lib/appState.svelte';
	import * as api from '$lib/tauriApi';
	import { isAppError } from '$lib/tauriApi';
	import { errorKindLabel, type ServerProfile } from '$lib/types';
	import { invalidate as invalidateWaypoints } from '$lib/waypointCache';

	let showAddForm = $state(false);
	let editingId = $state<string | null>(null);

	let formName = $state('');
	let formHost = $state('');
	let formPort = $state(7878);
	let formUsername = $state('');
	let formPassword = $state('');

	let dbEnabled = $state(false);
	let dbHost = $state('');
	let dbPort = $state(3306);
	let dbDatabase = $state('acore_world');
	let dbUsername = $state('gmconsole_ro');
	let dbPassword = $state('');

	let testResults = $state<Record<string, { pending: boolean; message: string; ok: boolean }>>({});
	let dbTestResults = $state<Record<string, { pending: boolean; message: string; ok: boolean }>>({});

	async function refresh() {
		appState.profiles = await api.listProfiles();
	}

	function resetForm() {
		formName = '';
		formHost = '';
		formPort = 7878;
		formUsername = '';
		formPassword = '';
		dbEnabled = false;
		dbHost = '';
		dbPort = 3306;
		dbDatabase = 'acore_world';
		dbUsername = 'gmconsole_ro';
		dbPassword = '';
		showAddForm = false;
		editingId = null;
	}

	function startEdit(p: ServerProfile) {
		editingId = p.id;
		formName = p.name;
		formHost = p.host;
		formPort = p.soapPort;
		formUsername = p.username;
		formPassword = '';
		if (p.db) {
			dbEnabled = true;
			dbHost = p.db.host;
			dbPort = p.db.port;
			dbDatabase = p.db.database;
			dbUsername = p.db.username;
			dbPassword = '';
		} else {
			dbEnabled = false;
			dbHost = '';
			dbPort = 3306;
			dbDatabase = 'acore_world';
			dbUsername = 'gmconsole_ro';
			dbPassword = '';
		}
		showAddForm = true;
	}

	async function submitForm() {
		const db = dbEnabled
			? { host: dbHost, port: dbPort, database: dbDatabase, username: dbUsername }
			: null;
		const dbPw = dbEnabled && dbPassword ? dbPassword : undefined;

		if (editingId) {
			await api.updateProfile({
				id: editingId,
				name: formName,
				host: formHost,
				soapPort: formPort,
				username: formUsername,
				password: formPassword || undefined,
				db,
				dbPassword: dbPw
			});
			invalidateWaypoints(editingId);
		} else {
			await api.addProfile({
				name: formName,
				host: formHost,
				soapPort: formPort,
				username: formUsername,
				password: formPassword,
				db,
				dbPassword: dbPw
			});
		}
		resetForm();
		await refresh();
	}

	async function remove(id: string) {
		await api.removeProfile(id);
		invalidateWaypoints(id);
		if (appState.activeProfileId === id) {
			appState.activeProfileId = null;
			appState.view = 'connections';
		}
		await refresh();
	}

	async function test(id: string) {
		testResults[id] = { pending: true, message: '', ok: false };
		try {
			const result = await api.testConnection(id);
			testResults[id] = { pending: false, ok: true, message: `OK (${result.latencyMs}ms)` };
		} catch (e) {
			const message = isAppError(e) ? errorKindLabel(e.kind) : String(e);
			testResults[id] = { pending: false, ok: false, message };
		}
	}

	async function testDb(id: string) {
		dbTestResults[id] = { pending: true, message: '', ok: false };
		try {
			const result = await api.testDbConnection(id);
			dbTestResults[id] = { pending: false, ok: true, message: `OK (${result.latencyMs}ms)` };
		} catch (e) {
			const message = isAppError(e) ? errorKindLabel(e.kind) : String(e);
			dbTestResults[id] = { pending: false, ok: false, message };
		}
	}

	function open(id: string) {
		appState.activeProfileId = id;
		appState.view = 'dashboard';
	}

	refresh();
</script>

<div class="manager">
	<div class="header">
		<h2>Server Connections</h2>
		<button onclick={() => { resetForm(); showAddForm = true; }}>+ Add Profile</button>
	</div>

	{#if showAddForm}
		<form class="profile-form" onsubmit={(e) => { e.preventDefault(); submitForm(); }}>
			<h3>{editingId ? 'Edit Profile' : 'New Profile'}</h3>
			<label>
				Name
				<input bind:value={formName} placeholder="e.g. Playerbots Realm" required />
			</label>
			<label>
				Host
				<input bind:value={formHost} placeholder="127.0.0.1" required />
			</label>
			<label>
				SOAP Port
				<input type="number" bind:value={formPort} required />
			</label>
			<label>
				GM Username
				<input bind:value={formUsername} placeholder="admin" required />
			</label>
			<label>
				Password
				<input
					type="password"
					bind:value={formPassword}
					placeholder={editingId ? 'Leave blank to keep existing' : ''}
					required={!editingId}
				/>
			</label>

			<details open={dbEnabled}>
				<summary>
					<label class="db-toggle">
						<input
							type="checkbox"
							bind:checked={dbEnabled}
							onclick={(e) => e.stopPropagation()}
						/>
						Database connection (optional — enables waypoint/item pickers)
					</label>
				</summary>
				{#if dbEnabled}
					<div class="db-fields">
						<label>
							DB Host
							<input bind:value={dbHost} placeholder="127.0.0.1" required={dbEnabled} />
						</label>
						<label>
							DB Port
							<input type="number" bind:value={dbPort} required={dbEnabled} />
						</label>
						<label>
							Database name
							<input bind:value={dbDatabase} placeholder="acore_world" required={dbEnabled} />
						</label>
						<label>
							DB Username
							<input bind:value={dbUsername} placeholder="gmconsole_ro" required={dbEnabled} />
						</label>
						<label>
							DB Password
							<input
								type="password"
								bind:value={dbPassword}
								placeholder={editingId ? 'Leave blank to keep existing' : ''}
							/>
						</label>
					</div>
				{/if}
			</details>

			<div class="form-actions">
				<button type="submit">Save</button>
				<button type="button" onclick={resetForm}>Cancel</button>
			</div>
		</form>
	{/if}

	<div class="profile-list">
		{#each appState.profiles as profile (profile.id)}
			<div class="profile-card">
				<div class="profile-info" onclick={() => open(profile.id)} role="button" tabindex="0"
					onkeydown={(e) => e.key === 'Enter' && open(profile.id)}>
					<strong>{profile.name}</strong>
					<span class="detail">{profile.host}:{profile.soapPort} · {profile.username}</span>
					{#if profile.db}
						<span class="detail db-detail">DB: {profile.db.host}:{profile.db.port}/{profile.db.database}</span>
					{/if}
				</div>
				<div class="profile-actions">
					<button onclick={() => test(profile.id)}>Test</button>
					{#if profile.db}
						<button onclick={() => testDb(profile.id)}>Test DB</button>
					{/if}
					<button onclick={() => startEdit(profile)}>Edit</button>
					<button class="danger" onclick={() => remove(profile.id)}>Remove</button>
				</div>
				{#if testResults[profile.id]}
					{@const r = testResults[profile.id]}
					<div class="test-result" class:ok={r.ok} class:pending={r.pending}>
						SOAP: {r.pending ? 'Testing…' : r.message}
					</div>
				{/if}
				{#if dbTestResults[profile.id]}
					{@const r = dbTestResults[profile.id]}
					<div class="test-result" class:ok={r.ok} class:pending={r.pending}>
						DB: {r.pending ? 'Testing…' : r.message}
					</div>
				{/if}
			</div>
		{:else}
			<p class="empty">No servers saved yet. Add one to get started.</p>
		{/each}
	</div>
</div>

<style>
	.manager {
		max-width: 640px;
		margin: 0 auto;
		padding: 1.5rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.profile-form {
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		background: var(--surface-raised, #f0f0f0);
		padding: 1rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.profile-form label {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		font-size: 0.85rem;
	}

	.db-toggle {
		display: inline-flex !important;
		flex-direction: row !important;
		align-items: center;
		gap: 0.4rem;
		cursor: pointer;
	}

	details summary {
		cursor: pointer;
		margin: 0.3rem 0;
	}

	.db-fields {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-top: 0.5rem;
		padding: 0.6rem;
		background: rgba(0, 0, 0, 0.03);
		border-radius: 6px;
	}

	.form-actions {
		display: flex;
		gap: 0.5rem;
	}

	.profile-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.profile-card {
		border: 1px solid rgba(128, 128, 128, 0.3);
		border-radius: 8px;
		padding: 0.75rem;
	}

	.profile-info {
		display: flex;
		flex-direction: column;
		cursor: pointer;
	}

	.detail {
		font-size: 0.8rem;
		opacity: 0.7;
	}

	.db-detail {
		opacity: 0.55;
	}

	.profile-actions {
		display: flex;
		gap: 0.4rem;
		margin-top: 0.5rem;
	}

	.danger {
		color: #d94848;
	}

	.test-result {
		margin-top: 0.4rem;
		font-size: 0.8rem;
	}

	.test-result.ok {
		color: #2a9d5c;
	}

	.test-result.pending {
		opacity: 0.6;
	}

	.empty {
		opacity: 0.6;
		font-style: italic;
	}
</style>
