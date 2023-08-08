<script lang="ts">
	import type { Reason } from './Reason';

	export let text: string;
	export let clicked: boolean = false;
</script>

{#if clicked}
	<button class="card p-4 flex grow justify-center rounded-md bg-success-600 dark:bg-success-800 break-all" on:click>
		{text}
	</button>
{:else}
	<button class="card p-4 flex grow justify-center rounded-md break-all" on:click>
		{text}
	</button>
{/if}
