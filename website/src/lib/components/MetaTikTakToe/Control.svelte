<script lang="ts">
	import type Grid from './Grid.svelte';

	export let grid: Grid;

	function reset() {
		grid.reset();
	}

	function connect() {
		grid.connect();
	}

	function new_game() {
		grid.new_game();
	}

	function join_game() {
		grid.join_game();
	}
</script>

<div class="card min-w-[20%] p-4 flex flex-col gap-4">
	<p class="font-bold">Control</p>
	<hr class="" />
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={reset}>Reset</button>
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={connect}>Connect</button>
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={new_game}>New Game</button>
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={join_game}>Join Game</button>
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={reset}>Reset</button>
</div>
