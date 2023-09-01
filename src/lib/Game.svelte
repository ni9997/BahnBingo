<script lang="ts">
	import Control from './Control.svelte';
	import Grid from './Grid.svelte';

	let grid: Grid;
	let control: Control;
</script>
<div class="flex items-center justify-center">
    <div class="m-4 grid gap-4 xl:flex">
        <Grid bind:this={grid} />
        <Control bind:this={control} {grid} />
    </div>
</div>

