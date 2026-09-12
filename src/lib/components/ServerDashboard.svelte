<script lang="ts">
	import ConsoleOutput from './ConsoleOutput.svelte';
	import WaypointPicker from './WaypointPicker.svelte';
	import ItemPicker from './ItemPicker.svelte';
	import { appState } from '$lib/appState.svelte';
	import * as api from '$lib/tauriApi';
	import { runAction } from '$lib/runAction';
	import { isAppError } from '$lib/tauriApi';
	import { errorKindLabel } from '$lib/types';

	let openForm = $state<string | null>(null);
	let showWaypointPicker = $state(false);
	let showItemPicker = $state(false);

	// form fields, shared scratch state reused across the small forms below
	let target = $state('');
	let itemId = $state('');
	let count = $state(1);
	let level = $state(1);
	let goldAmount = $state(0);
	let location = $state('');
	let mapId = $state(0);
	let coordX = $state(0);
	let coordY = $state(0);
	let coordZ = $state(0);
	let reason = $state('');
	let banDuration = $state('-1');
	let tableName = $state('creature_template');

	let statusMessage = $state<string | null>(null);
	let statusOk = $state(false);
	let statusPending = $state(false);

	function toggle(name: string) {
		openForm = openForm === name ? null : name;
	}

	async function checkStatus() {
		if (!appState.activeProfile) return;
		statusPending = true;
		try {
			const result = await api.testConnection(appState.activeProfile.id);
			statusOk = true;
			statusMessage = `Online — ${result.latencyMs}ms`;
		} catch (e) {
			statusOk = false;
			statusMessage = isAppError(e) ? errorKindLabel(e.kind) : String(e);
		} finally {
			statusPending = false;
		}
	}

	checkStatus();

	function profileId(): string {
		return appState.activeProfile!.id;
	}

	async function doRevive() {
		await runAction('action', `revive ${target}`.trim(), () => api.gmRevive(profileId(), target || undefined));
	}
	async function doAddItem() {
		await runAction('action', `additem ${itemId} ${count}`, () =>
			api.gmAddItem(profileId(), itemId, count)
		);
	}
	async function doSetLevel() {
		await runAction('action', `character level ${target} ${level}`, () =>
			api.gmSetLevel(profileId(), target, level)
		);
	}
	async function doSetGold() {
		await runAction('action', `modify money ${goldAmount}`, () =>
			api.gmSetGold(profileId(), goldAmount)
		);
	}
	async function doTeleportNamed() {
		await runAction('action', `tele name ${target} ${location}`, () =>
			api.gmTeleportNamed(profileId(), target, location)
		);
	}
	async function doTeleportCoords() {
		await runAction('action', `go xyz ${coordX} ${coordY} ${coordZ} ${mapId}`, () =>
			api.gmTeleportCoords(profileId(), mapId, coordX, coordY, coordZ)
		);
	}
	async function doKick() {
		await runAction('action', `kick ${target}`, () => api.gmKick(profileId(), target, reason || undefined));
	}
	async function doBan() {
		await runAction('action', `ban account ${target} ${banDuration} ${reason}`, () =>
			api.gmBanAccount(profileId(), target, banDuration, reason)
		);
	}
	async function doServerInfo() {
		await runAction('action', 'server info', () => api.gmServerInfo(profileId()));
	}
	async function doReload() {
		await runAction('action', `reload ${tableName}`, () => api.gmReloadTable(profileId(), tableName));
	}
</script>

{#if appState.activeProfile}
	<div class="dashboard">
		<div class="header">
			<div>
				<h2>{appState.activeProfile.name}</h2>
				<span class="detail">{appState.activeProfile.host}:{appState.activeProfile.soapPort}</span>
			</div>
			<div class="status">
				<button onclick={checkStatus} disabled={statusPending}>
					{statusPending ? 'Checking…' : 'Refresh Status'}
				</button>
				{#if statusMessage}
					<span class:ok={statusOk} class:bad={!statusOk}>{statusMessage}</span>
				{/if}
			</div>
		</div>

		<div class="body">
			<div class="actions">
				<button class="action-btn" onclick={doServerInfo}>Server Info</button>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('revive')}>Revive</button>
					{#if openForm === 'revive'}
						<div class="form">
							<input bind:value={target} placeholder="Target name (blank = self)" />
							<button onclick={doRevive}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('additem')}>Add Item</button>
					{#if openForm === 'additem'}
						<div class="form">
							<input bind:value={itemId} placeholder="Item ID or #name" />
							<input type="number" bind:value={count} min="1" />
							<div class="form-row">
								<button onclick={doAddItem}>Run</button>
								{#if appState.activeProfile?.db}
									<button
										type="button"
										onclick={() => (showItemPicker = !showItemPicker)}
									>
										Browse…
									</button>
								{/if}
							</div>
							{#if showItemPicker && appState.activeProfile?.db}
								<ItemPicker
									profileId={profileId()}
									onselect={(entry) => {
										itemId = String(entry);
										showItemPicker = false;
									}}
								/>
							{/if}
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('level')}>Set Level</button>
					{#if openForm === 'level'}
						<div class="form">
							<input bind:value={target} placeholder="Target name" />
							<input type="number" bind:value={level} min="1" max="255" />
							<button onclick={doSetLevel}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('gold')}>Set Gold</button>
					{#if openForm === 'gold'}
						<div class="form">
							<input type="number" bind:value={goldAmount} placeholder="Amount (copper)" />
							<button onclick={doSetGold}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('tele-name')}>Teleport (Named)</button>
					{#if openForm === 'tele-name'}
						<div class="form">
							<input bind:value={target} placeholder="Target name" />
							<input bind:value={location} placeholder="Location name" />
							<div class="form-row">
								<button onclick={doTeleportNamed}>Run</button>
								{#if appState.activeProfile?.db}
									<button
										type="button"
										onclick={() => (showWaypointPicker = !showWaypointPicker)}
									>
										Browse…
									</button>
								{/if}
							</div>
							{#if showWaypointPicker && appState.activeProfile?.db}
								<WaypointPicker
									profileId={profileId()}
									onselect={(name) => {
										location = name;
										showWaypointPicker = false;
									}}
								/>
							{/if}
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('tele-xyz')}>Teleport (Coords)</button>
					{#if openForm === 'tele-xyz'}
						<div class="form">
							<input type="number" bind:value={mapId} placeholder="Map ID" />
							<input type="number" bind:value={coordX} placeholder="X" />
							<input type="number" bind:value={coordY} placeholder="Y" />
							<input type="number" bind:value={coordZ} placeholder="Z" />
							<button onclick={doTeleportCoords}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('kick')}>Kick</button>
					{#if openForm === 'kick'}
						<div class="form">
							<input bind:value={target} placeholder="Target name" />
							<input bind:value={reason} placeholder="Reason (optional)" />
							<button onclick={doKick}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('ban')}>Ban Account</button>
					{#if openForm === 'ban'}
						<div class="form">
							<input bind:value={target} placeholder="Account name" />
							<input bind:value={banDuration} placeholder="Duration (-1 = permanent, 1d, 2h...)" />
							<input bind:value={reason} placeholder="Reason" />
							<button onclick={doBan}>Run</button>
						</div>
					{/if}
				</div>

				<div class="action-group">
					<button class="action-btn" onclick={() => toggle('reload')}>Reload Table</button>
					{#if openForm === 'reload'}
						<div class="form">
							<input bind:value={tableName} placeholder="Table name" />
							<button onclick={doReload}>Run</button>
						</div>
					{/if}
				</div>
			</div>

			<div class="log">
				<ConsoleOutput />
			</div>
		</div>
	</div>
{/if}

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		height: 100%;
		padding: 1rem;
		gap: 1rem;
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
	}

	.detail {
		font-size: 0.8rem;
		opacity: 0.7;
	}

	.status {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
	}

	.status .ok {
		color: #2a9d5c;
	}

	.status .bad {
		color: #d94848;
	}

	.body {
		display: flex;
		flex: 1;
		min-height: 0;
		gap: 1rem;
	}

	.actions {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
		width: 320px;
		overflow-y: auto;
	}

	.action-btn {
		width: 100%;
		text-align: left;
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: 0.3rem;
		padding: 0.4rem;
		background: var(--surface-raised, #f0f0f0);
		border-radius: 6px;
		margin-top: 0.2rem;
	}

	.form-row {
		display: flex;
		gap: 0.3rem;
	}

	.log {
		flex: 1;
		min-width: 0;
	}
</style>
