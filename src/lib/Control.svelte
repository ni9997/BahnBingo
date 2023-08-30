<script lang="ts">
	import type Grid from './Grid.svelte';

	export let grid: Grid;

	function reset() {
		grid.reset();
	}
</script>

<div class="flex justify-center items-center gap-4">
	<!-- <h3>Game Seed: 1234</h3> -->
	<button class="btn variant-outline flex-grow" on:click={reset}>Reset</button>
</div>
