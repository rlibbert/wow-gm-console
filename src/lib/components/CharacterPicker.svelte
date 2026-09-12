<script lang="ts">
	import * as api from '$lib/tauriApi';
	import { isAppError } from '$lib/tauriApi';
	import { errorKindLabel, type CharacterSummary } from '$lib/types';
	import { raceName, className, classColor } from '$lib/characterInfo';

	let { profileId, onselect }: { profileId: string; onselect: (name: string) => void } = $props();

	let nameSubstring = $state('');
	let onlineOnly = $state(true);

	let results = $state<CharacterSummary[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let searched = $state(false);

	let debounceHandle: ReturnType<typeof setTimeout> | undefined;

	function triggerSearch(debounceMs = 300) {
		clearTimeout(debounceHandle);
		debounceHandle = setTimeout(runSearch, debounceMs);
	}

	async function runSearch() {
		const hasNameFilter = nameSubstring.trim().length >= 2;
		if (!hasNameFilter && !onlineOnly) {
			results = [];
			searched = false;
			return;
		}

		loading = true;
		error = null;
		searched = true;
		try {
			results = await api.searchCharacters(
				profileId,
				{ nameSubstring: nameSubstring.trim() || undefined, onlineOnly },
				100
			);
		} catch (e) {
			error = isAppError(e) ? errorKindLabel(e.kind) : String(e);
			results = [];
		} finally {
			loading = false;
		}
	}

	// Online-only defaults on, which already satisfies the search guard --
	// run an initial search so opening the picker shows something right away.
	runSearch();
</script>

<div class="picker">
	<div class="filters">
		<input
			bind:value={nameSubstring}
			oninput={() => triggerSearch()}
			placeholder="Search by name (2+ chars)..."
			autocomplete="off"
		/>
		<label class="online-toggle">
			<input type="checkbox" bind:checked={onlineOnly} onchange={() => triggerSearch(0)} />
			Online only
		</label>
	</div>

	{#if loading}
		<p class="status">Searching…</p>
	{:else if error}
		<p class="status error">{error}</p>
	{:else if !searched}
		<p class="status">Type a name or enable "Online only" to search.</p>
	{:else if results.length === 0}
		<p class="status">No matching characters.</p>
	{:else}
		<ul class="results">
			{#each results as ch (ch.guid)}
				<li>
					<button class="result-row" onclick={() => onselect(ch.name)}>
						<span class="name" style="color: {classColor(ch.class)}">
							{ch.name}
							{#if ch.online}<span class="online-dot" title="Online">●</span>{/if}
						</span>
						<span class="meta">lvl {ch.level} {raceName(ch.race)} {className(ch.class)}</span>
					</button>
				</li>
			{/each}
		</ul>
		{#if results.length === 100}
			<p class="status">Showing first 100 matches — refine your search for more precision.</p>
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

	.filters {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.filters input:not([type]) {
		flex: 1;
	}

	.online-toggle {
		display: inline-flex;
		align-items: center;
		gap: 0.3rem;
		font-size: 0.8rem;
		white-space: nowrap;
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
		flex-direction: column;
		align-items: flex-start;
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
		font-weight: 500;
	}

	.online-dot {
		color: #2a9d5c;
		font-size: 0.6rem;
		margin-left: 0.2rem;
	}

	.meta {
		font-size: 0.72rem;
		opacity: 0.7;
	}
</style>
