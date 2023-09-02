<script lang="ts">
	import IconCircle from '@tabler/icons-svelte/dist/svelte/icons/IconCircle.svelte';
	import IconX from '@tabler/icons-svelte/dist/svelte/icons/IconX.svelte';

	var state: string[][] = [
		['', '', ''],
		['', '', ''],
		['', '', '']
	];
	var winner: string = '';

	function place(i: number, j: number) {
		if (i % 2 == 0) {
			state[i][j] = 'X';
		} else {
			state[i][j] = 'O';
		}

		// check_win();
		if (i % 2 == 0) {
			winner = 'X';
		} else {
			winner = 'O';
		}
	}

	function check_win() {
		winner = 'X';
	}
</script>

<div class="flex flex-1 md:max-w-2xl bg-green-600 relative">
	{#if winner == 'O'}
		<div class="absolute z-10 justify-center items-center flex flex-1 h-full w-full">
			<IconCircle class="stroke-red-600 h-full w-full" />
		</div>
	{:else if winner == 'X'}
		<div class="absolute z-10 justify-center items-center flex flex-1 h-full w-full">
			<IconX class="stroke-red-600 h-full w-full" />
		</div>
	{/if}
	{#each [0, 1, 2] as i}
		<div class="flex flex-col flex-1">
			{#each [0, 1, 2] as j}
				<button
					class="card m-1 aspect-square border-solid border border-red-600 rounded-none flex items-center justify-center h-full"
					on:click={() => {
						place(i, j);
					}}
				>
					{state[i][j]}
				</button>
			{/each}
		</div>
	{/each}
</div>
