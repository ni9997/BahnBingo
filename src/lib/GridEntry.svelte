<script lang="ts">
	import type { Reason } from './Reason';

	export let data: Reason;

	let clicked: boolean = false;

	function handleClick(e: MouseEvent) {
		console.log(e);
		clicked = !clicked;
	}
</script>

<div class="flex grow">
	{#if clicked}
		<button
			class="card p-4 flex grow justify-center rounded-md bg-success-600"
			on:click={handleClick}
			>{data.text}
		</button>
	{:else}
	<button
			class="card p-4 flex grow justify-center rounded-md"
			on:click={handleClick}
			>{data.text}
		</button>
	{/if}

</div>
