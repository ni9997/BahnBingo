<script lang="ts">
	// import Grid from

	let state: boolean[][] = []

	for (let i = 0; i < 5; i++) {
		state[i] = []
		for (let j = 0; j < 5; j++) {
			state[i][j]	= false;
		}
	}

	class Reason {
		id: number;
		text: string;

		constructor(id: number, text: string) {
			this.id = id;
			this.text = text
		}
	}

	let reasons: Reason[] = []

	for (let index = 0; index < 25; index++) {
		reasons.push(new Reason(index, `Reason ${index}`))
	}

	function handleClick() {
		// console.log(e);
		console.log(state);
	}

</script>

<div class="m-4 container mx-auto flex grow justify-center">
	<div class="grid grid-cols-5 grid-rows-5 w-full flex mx-auto  grow justify-center gap-2">
		{#each reasons as r, index(r.id)}
			<button class="card p-4 flex grow justify-center" on:click={handleClick}>{r.text}</button>
		{/each}
	</div>
</div>
