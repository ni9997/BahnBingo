<script lang="ts">
	import TicTacToe from '../TicTacToe/TicTacToe.svelte';
	import { Player } from '../TicTacToe/utils';

	let boards: TicTacToe[][] = [Array(3), Array(3), Array(3)];

	let current_player: Player = Player.O;

	let socket: WebSocket;

	export function reset() {
		for (let i = 0; i < 3; i++) {
			for (let j = 0; j < 3; j++) {
				boards[i][j].reset();
			}
		}
	}

	export function connect() {
		console.log("TEST");
		
		socket = new WebSocket("ws://localhost:8080")

		socket.addEventListener("open", (event) => {
			socket.send("Test");
		});

		socket.addEventListener("message", (event) => {
			console.log("Received: ", event.data);
		});

		socket.addEventListener("error", (event) => {
			console.log(event);
		})
	}

	function send() {
		socket.send("test2")
	}
</script>

<div class="m-4 grid gap-4 xl:flex flex-1">
	<div class="flex flex-1 m-4 md:max-w-2xl bg-green-600">
		{#each [0, 1, 2] as i}
			<div class="flex flex-col flex-1">
				{#each [0, 1, 2] as j}
					<div class="card m-1 aspect-square border-solid border border-red-600 rounded-none flex">
						<TicTacToe standalone={false} bind:current_player bind:this={boards[i][j]} />
					</div>
				{/each}
			</div>
		{/each}
	</div>
</div>
<button class="btn variant-filled" on:click={connect}>CLICK</button>
<button class="btn variant-filled" on:click={send}>CLICK</button>
