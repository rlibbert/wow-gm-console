<script lang="ts">
	import { appState } from '$lib/appState.svelte';
	import * as api from '$lib/tauriApi';
	import { isAppError } from '$lib/tauriApi';
	import { errorKindLabel, type ServerProfile } from '$lib/types';

	let showAddForm = $state(false);
	let editingId = $state<string | null>(null);

	let formName = $state('');
	let formHost = $state('');
	let formPort = $state(7878);
	let formUsername = $state('');
	let formPassword = $state('');

	let testResults = $state<Record<string, { pending: boolean; message: string; ok: boolean }>>({});

	async function refresh() {
		appState.profiles = await api.listProfiles();
	}

	function resetForm() {
		formName = '';
		formHost = '';
		formPort = 7878;
		formUsername = '';
		formPassword = '';
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
		showAddForm = true;
	}

	async function submitForm() {
		if (editingId) {
			await api.updateProfile({
				id: editingId,
				name: formName,
				host: formHost,
				soapPort: formPort,
				username: formUsername,
				password: formPassword || undefined
			});
		} else {
			await api.addProfile({
				name: formName,
				host: formHost,
				soapPort: formPort,
				username: formUsername,
				password: formPassword
			});
		}
		resetForm();
		await refresh();
	}

	async function remove(id: string) {
		await api.removeProfile(id);
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
				</div>
				<div class="profile-actions">
					<button onclick={() => test(profile.id)}>Test</button>
					<button onclick={() => startEdit(profile)}>Edit</button>
					<button class="danger" onclick={() => remove(profile.id)}>Remove</button>
				</div>
				{#if testResults[profile.id]}
					{@const r = testResults[profile.id]}
					<div class="test-result" class:ok={r.ok} class:pending={r.pending}>
						{r.pending ? 'Testing…' : r.message}
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
