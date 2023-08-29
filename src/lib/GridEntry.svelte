<script lang="ts">
	export let text: string;
	export let clicked = false;
</script>

{#if clicked}
	<button
		class="card p-4 flex grow justify-center rounded-md bg-success-600 dark:bg-success-800 sm:break-normal break-normal"
		on:click
	>
		{text}
	</button>
{:else}
	<button class="card p-4 flex grow justify-center rounded-md sm:break-normal break-all" on:click>
		{text}
	</button>
{/if}
