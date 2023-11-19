<script lang="ts">
	import type Grid from './Grid.svelte';

	export let grid: Grid;
	export let socket: WebSocket;

	// grid.set_control(this);

	let session_id: string;

	function reset() {
		grid.reset();
	}

	// function connect() {
	// 	grid.connect();
	// }

	function new_game() {
		grid.new_game();
	}

	function join_game() {
		grid.join_game(session_id);
	}

	function start_game() {
		grid.start_game();
	}

	function send_move() {}
</script>

<div class="card min-w-[20%] p-4 flex flex-col gap-4">
	<p class="font-bold">Control</p>
	<hr class="" />
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={reset}>Reset</button>
	<!-- <button class="btn variant-filled-success flex flex-1 w-full" on:click={connect}>Connect</button> -->
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={new_game}
		>Create new game</button
	>
	<div class="flex flex-col gap-2">
		<input type="text" class="input px-2" placeholder="Session ID" bind:value={session_id} />
		<button class="btn variant-filled-success flex flex-1 w-full" on:click={join_game}
			>Join Game</button
		>
	</div>
	<button class="btn variant-filled-success flex flex-1 w-full" on:click={start_game}
		>Start Game</button
	>
</div>
