<script lang="ts">
	import Control from './Control.svelte';
	import Grid from './Grid.svelte';

	let grid: Grid;
	let control: Control;
</script>

<div class="m-4 grid gap-4">
	<Grid bind:this={grid} />
	<Control bind:this={control} {grid} />
</div>
