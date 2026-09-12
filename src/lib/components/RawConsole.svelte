<script lang="ts">
	import ConsoleOutput from './ConsoleOutput.svelte';
	import { appState } from '$lib/appState.svelte';
	import * as api from '$lib/tauriApi';
	import { runAction } from '$lib/runAction';

	let input = $state('');
	let history: string[] = $state([]);
	let historyIndex = $state(-1);

	async function submit() {
		const command = input.trim();
		if (!command || !appState.activeProfile) return;

		history.push(command);
		historyIndex = history.length;
		input = '';

		await runAction('console', command, () =>
			api.sendRawCommand(appState.activeProfile!.id, command)
		);
	}

	function onKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			submit();
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			if (historyIndex > 0) {
				historyIndex--;
				input = history[historyIndex];
			}
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			if (historyIndex < history.length - 1) {
				historyIndex++;
				input = history[historyIndex];
			} else {
				historyIndex = history.length;
				input = '';
			}
		}
	}
</script>

<div class="console-view">
	<div class="output-area">
		<ConsoleOutput />
	</div>
	<div class="input-row">
		<span class="prompt">.</span>
		<input
			bind:value={input}
			onkeydown={onKeydown}
			placeholder="revive Jaarl, server info, additem 2825 1, ..."
			autocomplete="off"
			spellcheck="false"
		/>
		<button onclick={submit}>Send</button>
	</div>
</div>

<style>
	.console-view {
		display: flex;
		flex-direction: column;
		height: 100%;
		gap: 0.5rem;
	}

	.output-area {
		flex: 1;
		min-height: 0;
	}

	.input-row {
		display: flex;
		align-items: center;
		gap: 0.4rem;
	}

	.prompt {
		font-family: monospace;
		opacity: 0.6;
	}

	input {
		flex: 1;
		font-family: monospace;
	}
</style>
