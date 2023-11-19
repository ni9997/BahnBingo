<script lang="ts">
	import type Control from './Control.svelte';
	import TicTacToe from '../TicTacToe/TicTacToe.svelte';
	import { Player } from '../TicTacToe/utils';
	import { getToastStore } from '@skeletonlabs/skeleton';

	let boards: TicTacToe[][] = [Array(3), Array(3), Array(3)];

	const toastStore = getToastStore();

	export let control: Control;

	// export function set_control(c: Control) {
	// 	control = c;
	// }

	let current_player: Player = Player.O;
	let this_player: Player;

	export let socket: WebSocket;

	enum MessageType {
		NewSession = 'NewSession'
	}

	class Message {
		message_type: MessageType;

		constructor() {
			this.message_type = MessageType.NewSession;
		}
	}

	export function reset() {
		for (let i = 0; i < 3; i++) {
			for (let j = 0; j < 3; j++) {
				boards[i][j].reset();
			}
		}
	}

	socket.addEventListener('message', (event) => {
		console.log('Received: ', event.data);
		let data = JSON.parse(event.data);
		console.log(data);
		if (data['MakeMove']) {
			let move = data['MakeMove'];
			// console.log(data["MakeMove"]);
			boards[move['global_grid_x']][move['global_grid_y']].remote_place(
				move['local_grid_x'],
				move['local_grid_y']
			);
		} else if (data['JoinSuccess']) {
			toastStore.trigger({ message: `Joined session ${data['JoinSuccess']}` });
		} else if (data['GameStart']) {
			if (data['GameStart']['starting_player'] == 'O') {
				current_player = Player.O;
			} else {
				current_player = Player.X;
			}
			if (data['GameStart']['player'] == 'O') {
				this_player = Player.O;
			} else {
				this_player = Player.X;
			}
			toastStore.trigger({ message: 'Game starting now!' });
		}
	});

	export function new_game() {
		socket.send('{"NewSession":null}');
	}

	export function join_game(session_id: string) {
		socket.send(`{"JoinSession":"${session_id}"}`);
	}

	export function start_game() {
		socket.send('"RequestGameStart"');
	}

	let test_text = 'TEST';
</script>

<div class="m-4 grid gap-4 xl:flex flex-1">
	<div class="flex flex-1 m-4 md:max-w-2xl bg-green-600">
		{#each [0, 1, 2] as i}
			<div class="flex flex-col flex-1">
				{#each [0, 1, 2] as j}
					<div class="card m-1 aspect-square border-solid border border-red-600 rounded-none flex">
						<TicTacToe
							standalone={false}
							bind:current_player
							bind:this_player
							bind:this={boards[i][j]}
							{socket}
							global_x={i}
							global_y={j}
						/>
					</div>
				{/each}
			</div>
		{/each}
	</div>
</div>
<!-- <button class="btn variant-filled" on:click={connect}>Connect</button>
<button class="btn variant-filled" on:click={send}>Send</button>
<textarea bind:value={test_text}></textarea> -->
