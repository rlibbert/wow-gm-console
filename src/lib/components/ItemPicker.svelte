<script lang="ts">
	import * as api from '$lib/tauriApi';
	import { isAppError } from '$lib/tauriApi';
	import { errorKindLabel, type ItemSummary } from '$lib/types';
	import { ITEM_CLASSES, ITEM_QUALITIES, itemClassLabel, qualityColor } from '$lib/itemClasses';

	let { profileId, onselect }: { profileId: string; onselect: (entry: number) => void } = $props();

	let nameSubstring = $state('');
	let classFilter = $state<number | ''>('');
	let subclassFilter = $state<number | ''>('');
	let qualityMin = $state<number | ''>('');
	let requiredLevelMax = $state<number | ''>('');

	let results = $state<ItemSummary[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let searched = $state(false);

	let debounceHandle: ReturnType<typeof setTimeout> | undefined;

	let subclassOptions = $derived(
		classFilter === '' ? {} : (ITEM_CLASSES[classFilter]?.subclasses ?? {})
	);

	function onClassChange() {
		subclassFilter = '';
		triggerSearch(0);
	}

	function triggerSearch(debounceMs = 300) {
		clearTimeout(debounceHandle);
		debounceHandle = setTimeout(runSearch, debounceMs);
	}

	async function runSearch() {
		const hasNameFilter = nameSubstring.trim().length >= 2;
		const hasClassFilter = classFilter !== '';
		if (!hasNameFilter && !hasClassFilter) {
			results = [];
			searched = false;
			return;
		}

		loading = true;
		error = null;
		searched = true;
		try {
			results = await api.searchItems(profileId, {
				nameSubstring: nameSubstring.trim() || undefined,
				class: classFilter === '' ? undefined : classFilter,
				subclass: subclassFilter === '' ? undefined : subclassFilter,
				qualityMin: qualityMin === '' ? undefined : qualityMin,
				requiredLevelMax: requiredLevelMax === '' ? undefined : requiredLevelMax
			}, 100);
		} catch (e) {
			error = isAppError(e) ? errorKindLabel(e.kind) : String(e);
			results = [];
		} finally {
			loading = false;
		}
	}
</script>

<div class="picker">
	<div class="filters">
		<input
			bind:value={nameSubstring}
			oninput={() => triggerSearch()}
			placeholder="Search by name (2+ chars)..."
			autocomplete="off"
		/>
		<select bind:value={classFilter} onchange={onClassChange}>
			<option value="">Any class</option>
			{#each Object.entries(ITEM_CLASSES) as [id, info] (id)}
				<option value={Number(id)}>{info.name}</option>
			{/each}
		</select>
		<select bind:value={subclassFilter} onchange={() => triggerSearch(0)} disabled={classFilter === ''}>
			<option value="">Any subclass</option>
			{#each Object.entries(subclassOptions) as [id, name] (id)}
				<option value={Number(id)}>{name}</option>
			{/each}
		</select>
		<select bind:value={qualityMin} onchange={() => triggerSearch(0)}>
			<option value="">Any quality</option>
			{#each ITEM_QUALITIES as name, i (i)}
				<option value={i}>{name}+</option>
			{/each}
		</select>
		<input
			type="number"
			bind:value={requiredLevelMax}
			oninput={() => triggerSearch(0)}
			placeholder="Max req. level"
			min="1"
			max="255"
		/>
	</div>

	{#if loading}
		<p class="status">Searching…</p>
	{:else if error}
		<p class="status error">{error}</p>
	{:else if !searched}
		<p class="status">Select a class or type a name to search.</p>
	{:else if results.length === 0}
		<p class="status">No matching items.</p>
	{:else}
		<ul class="results">
			{#each results as item (item.entry)}
				<li>
					<button class="result-row" onclick={() => onselect(item.entry)}>
						<span class="name" style="color: {qualityColor(item.quality)}">{item.name}</span>
						<span class="meta">
							{itemClassLabel(item.class, item.subclass)} · ilvl {item.itemLevel} · req {item.requiredLevel}
						</span>
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
		max-height: 380px;
	}

	.filters {
		display: flex;
		flex-wrap: wrap;
		gap: 0.3rem;
	}

	.filters input,
	.filters select {
		flex: 1;
		min-width: 100px;
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
		max-height: 260px;
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

	.meta {
		font-size: 0.72rem;
		opacity: 0.7;
	}
</style>
