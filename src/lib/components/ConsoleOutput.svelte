<script lang="ts">
	import { log } from '$lib/log.svelte';
	import { errorKindLabel } from '$lib/types';

	let container: HTMLDivElement;

	$effect(() => {
		// scroll to bottom whenever a new entry lands
		log.entries.length;
		if (container) container.scrollTop = container.scrollHeight;
	});
</script>

<div class="output" bind:this={container}>
	{#if log.entries.length === 0}
		<p class="empty">No commands sent yet.</p>
	{/if}
	{#each log.entries as entry (entry.id)}
		<div class="entry" class:error={!!entry.error}>
			<div class="meta">
				<span class="time">{entry.timestamp.toLocaleTimeString()}</span>
				<span class="source">{entry.source}</span>
			</div>
			<div class="command">&gt; {entry.command}</div>
			{#if entry.error}
				<div class="result error-text">{errorKindLabel(entry.error.kind)}: {entry.error.message}</div>
			{:else if entry.result !== undefined}
				<div class="result">{entry.result}</div>
			{/if}
		</div>
	{/each}
</div>

<style>
	.output {
		font-family: 'SF Mono', Consolas, Menlo, monospace;
		font-size: 0.85rem;
		background: var(--surface-sunken, #1a1a1a);
		color: var(--text-console, #d0d0d0);
		border-radius: 8px;
		padding: 0.75rem;
		height: 100%;
		overflow-y: auto;
		white-space: pre-wrap;
	}

	.empty {
		opacity: 0.5;
		font-style: italic;
	}

	.entry {
		margin-bottom: 0.75rem;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.08);
	}

	.meta {
		display: flex;
		gap: 0.5rem;
		font-size: 0.75rem;
		opacity: 0.5;
		margin-bottom: 0.15rem;
	}

	.source {
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.command {
		color: #7fd1ff;
	}

	.result {
		margin-top: 0.15rem;
	}

	.error-text {
		color: #ff8080;
	}
</style>
