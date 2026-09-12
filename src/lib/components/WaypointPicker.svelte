<script lang="ts">
	import { getTeleports, invalidate } from '$lib/waypointCache';
	import type { Teleport } from '$lib/types';

	let { profileId, onselect }: { profileId: string; onselect: (name: string) => void } = $props();

	let query = $state('');
	let teleports = $state<Teleport[]>([]);
	let loading = $state(true);
	let error = $state<string | null>(null);

	async function load(forceRefresh = false) {
		loading = true;
		error = null;
		try {
			teleports = await getTeleports(profileId, forceRefresh);
		} catch (e) {
			error = String(e);
		} finally {
			loading = false;
		}
	}

	load();

	function refresh() {
		invalidate(profileId);
		load(true);
	}

	let filtered = $derived(
		query.trim() === ''
			? teleports
			: teleports.filter((t) => t.name.toLowerCase().includes(query.trim().toLowerCase()))
	);
</script>

<div class="picker">
	<div class="picker-header">
		<input
			bind:value={query}
			placeholder="Search {teleports.length} locations..."
			autocomplete="off"
		/>
		<button onclick={refresh} disabled={loading} title="Refresh from server">⟳</button>
	</div>

	{#if loading}
		<p class="status">Loading locations…</p>
	{:else if error}
		<p class="status error">{error}</p>
	{:else if filtered.length === 0}
		<p class="status">No matching locations.</p>
	{:else}
		<ul class="results">
			{#each filtered.slice(0, 200) as tp (tp.id)}
				<li>
					<button class="result-row" onclick={() => onselect(tp.name)}>
						<span class="name">{tp.name}</span>
						<span class="meta">map {tp.map}</span>
					</button>
				</li>
			{/each}
		</ul>
		{#if filtered.length > 200}
			<p class="status">Showing first 200 of {filtered.length} matches — refine your search.</p>
		{/if}
	{/if}
</div>

<style>
	.picker {
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
		background: var(--surface-raised, #f0f0f0);
		border-radius: 6px;
		padding: 0.5rem;
		margin-top: 0.3rem;
		max-height: 320px;
	}

	.picker-header {
		display: flex;
		gap: 0.3rem;
	}

	.picker-header input {
		flex: 1;
	}

	.status {
		font-size: 0.8rem;
		opacity: 0.7;
		margin: 0.2rem 0;
	}

	.status.error {
		color: #d94848;
		opacity: 1;
	}

	.results {
		list-style: none;
		margin: 0;
		padding: 0;
		overflow-y: auto;
		max-height: 240px;
	}

	.result-row {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: none;
		border: none;
		text-align: left;
		padding: 0.3rem 0.4rem;
		border-radius: 4px;
		cursor: pointer;
	}

	.result-row:hover {
		background: rgba(100, 108, 255, 0.15);
	}

	.name {
		font-size: 0.85rem;
	}

	.meta {
		font-size: 0.75rem;
		opacity: 0.6;
	}
</style>
