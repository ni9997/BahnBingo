<script lang="ts">
	import Control from './Control.svelte';
	import Grid from './Grid.svelte';

	let grid: Grid;
	let control: Control;
</script>

<div class="flex items-center justify-center h-full w-full bg-yellow-400 min-w-[500px]">
	<Grid bind:this={grid} />
	<Control bind:this={control} {grid} />
</div>
