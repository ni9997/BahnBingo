<script lang="ts">
	import type Grid from './Grid.svelte';

	export let grid: Grid;

	function reset() {
		grid.reset();
	}
</script>

<div class="card min-w-[20%] p-4">
	<p class="font-bold">Control</p>
	<hr class="my-4" />
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={reset}>Reset</button>
</div>
