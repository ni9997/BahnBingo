<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import Control from './Control.svelte';
	import Grid from './Grid.svelte';

	let grid: Grid;
	let control: Control;

	let socket: WebSocket;
	connect();
	setInterval(connect, 5000);

	let connected = false;

	function connect() {
		if (socket === undefined || (socket && socket.readyState == WebSocket.CLOSED)) {
			socket = new WebSocket('ws://localhost:8080');

			socket.addEventListener('open', (event) => {
				console.log(event);
				console.log(socket.readyState);
				connected = true;
			});

			socket.addEventListener('close', (event) => {
				connected = false;
			});
			socket.addEventListener('error', (event) => {
				connected = false;
			});
		}
	}
</script>

{#if connected}
	<div class="flex items-center justify-center h-full w-full bg-yellow-400 min-w-[500px] p-4">
		<Grid bind:this={grid} bind:socket bind:control />
		<Control bind:this={control} bind:socket {grid} />
	</div>
{:else}
	<div class="flex justify-center items-center h-full">
		<ProgressRadial />
	</div>
{/if}
