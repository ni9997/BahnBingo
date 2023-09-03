<script lang="ts">
	import IconCircle from '@tabler/icons-svelte/dist/svelte/icons/IconCircle.svelte';
	import IconX from '@tabler/icons-svelte/dist/svelte/icons/IconX.svelte';
	import { Player } from './utils';

	export let standalone = false;

	export let current_player: Player = Player.O;

	let margin = standalone ? 'm-0' : '';

	var state: Player[][] = [
		[Player.None, Player.None, Player.None],
		[Player.None, Player.None, Player.None],
		[Player.None, Player.None, Player.None]
	];

	export var winner = Player.None;
	export let finished = false;

	function place(i: number, j: number) {
		if (state[i][j] != Player.None) {
			return;
		}
		state[i][j] = current_player;

		check_win();
		// if (i % 2 == 0) {
		// 	winner = Player.X;
		// } else {
		// 	winner = Player.O;
		// }
		if (current_player == Player.O) {
			current_player = Player.X;
		} else if (current_player == Player.X) {
			current_player = Player.O;
		}
	}

	export function check_win() {
		for (let i = 0; i < 3; i++) {
			if (state[i][0] != Player.None && state[i][0] == state[i][1] && state[i][1] == state[i][2]) {
				winner = state[i][0];
				finished = true;
				return;
			} else if (
				state[0][i] != Player.None &&
				state[0][i] == state[1][i] &&
				state[1][i] == state[2][i]
			) {
				winner = state[0][i];
				finished = true;
				return;
			}
		}
		if (state[0][0] != Player.None && state[0][0] == state[1][1] && state[1][1] == state[2][2]) {
			winner = state[0][0];
			finished = true;
			return;
		} else if (
			state[0][2] != Player.None &&
			state[0][2] == state[1][1] &&
			state[1][1] == state[2][0]
		) {
			winner = state[0][2];
			finished = true;
			return;
		}
	}

	export function reset() {
		winner = Player.None;
		finished = false;
		for (let i = 0; i < 3; i++) {
			for (let j = 0; j < 3; j++) {
				state[i][j] = Player.None;
			}
		}
	}
</script>

<div class="flex flex-1 bg-green-600 relative w-full h-full aspect-square {margin}">
	{#if winner == Player.O}
		<div class="absolute z-10 justify-center items-center flex flex-1 h-full w-full">
			<IconCircle class="stroke-red-600 h-full w-full" />
		</div>
	{:else if winner == Player.X}
		<div class="absolute z-10 justify-center items-center flex flex-1 h-full w-full">
			<IconX class="stroke-blue-600 h-full w-full" />
		</div>
	{/if}
	{#each [0, 1, 2] as i}
		<div class="flex flex-col flex-1">
			{#each [0, 1, 2] as j}
				<button
					class="card m-1 aspect-square rounded-none flex items-center justify-center h-full"
					on:click={() => {
						place(i, j);
					}}
					disabled={finished}
				>
					{#if state[i][j] == Player.O}
						<IconCircle class="stroke-red-600 h-full w-full" />
					{:else if state[i][j] == Player.X}
						<IconX class="stroke-blue-600 h-full w-full" />
					{/if}
				</button>
			{/each}
		</div>
	{/each}
</div>
