<script lang="ts">
	import ConnectionManager from '$lib/components/ConnectionManager.svelte';
	import ServerDashboard from '$lib/components/ServerDashboard.svelte';
	import RawConsole from '$lib/components/RawConsole.svelte';
	import { appState } from '$lib/appState.svelte';
</script>

<div class="app">
	<nav>
		<button
			class:active={appState.view === 'connections'}
			onclick={() => (appState.view = 'connections')}
		>
			Servers
		</button>
		{#if appState.activeProfile}
			<button
				class:active={appState.view === 'dashboard'}
				onclick={() => (appState.view = 'dashboard')}
			>
				Dashboard
			</button>
			<button
				class:active={appState.view === 'console'}
				onclick={() => (appState.view = 'console')}
			>
				Console
			</button>
			<span class="active-server">{appState.activeProfile.name}</span>
		{/if}
	</nav>

	<main>
		{#if appState.view === 'connections'}
			<ConnectionManager />
		{:else if appState.view === 'dashboard' && appState.activeProfile}
			<ServerDashboard />
		{:else if appState.view === 'console' && appState.activeProfile}
			<div class="console-page"><RawConsole /></div>
		{/if}
	</main>
</div>

<style>
	:global(html, body) {
		height: 100%;
		margin: 0;
	}

	:global(*) {
		box-sizing: border-box;
	}

	.app {
		display: flex;
		flex-direction: column;
		height: 100vh;
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
	}

	nav {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-bottom: 1px solid rgba(128, 128, 128, 0.3);
		flex-shrink: 0;
	}

	nav button {
		background: none;
		border: none;
		padding: 0.4rem 0.8rem;
		cursor: pointer;
		border-radius: 6px;
	}

	nav button.active {
		background: rgba(100, 108, 255, 0.15);
		font-weight: 600;
	}

	.active-server {
		margin-left: auto;
		font-size: 0.85rem;
		opacity: 0.7;
	}

	main {
		flex: 1;
		min-height: 0;
		overflow: auto;
	}

	.console-page {
		height: 100%;
		padding: 1rem;
	}
</style>
