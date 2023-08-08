<script lang="ts">
	import { modalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import GridEntry from './GridEntry.svelte';
	import type { Reason } from './Reason';
	import { getReasons } from './ReasonBuilder';

	const winModal: ModalSettings = {
		type: 'alert',
		title: 'You "won"',
		body: 'At least something worked',
        buttonTextCancel: 'Replay',
		response(r) {
			reset();
		}
	};

	let reasons: Reason[] = getReasons();
	reasons = reasons.sort(() => 0.5 - Math.random()).slice(0, 25);

	export function update(e: MouseEvent, i: number) {
		// console.log(e.target);
		// console.log(i);
		reasons[i].clicked = !reasons[i].clicked;

		for (let index = 0; index < 5; index++) {
			let count = 0;
			for (let j = 0; j < 5; j++) {
				if (reasons[index + 5 * j].clicked) {
					count++;
				}
			}
			// console.log(count);
			if (count == 5) {
				finished();
				return;
			}
		}
		for (let index = 0; index < 5; index++) {
			let count = 0;
			for (let j = 0; j < 5; j++) {
				if (reasons[index * 5 + j].clicked) {
					count++;
				}
			}
			// console.log(count);
			if (count == 5) {
				finished();
				return;
			}
		}

		let count = 0;
		for (let index = 0; index < 5; index++) {
			// console.log(index * 5 + index);
			if (reasons[index * 5 + index].clicked) {
				count++;
			}
		}
		// console.log(`Diag pos: ${count}`);
		if (count == 5) {
			finished();
			return;
		}

		count = 0;
		for (let index = 0; index < 5; index++) {
			if (reasons[4 + index * 4].clicked) {
				count++;
			}
		}
		// console.log(`Diag neg: ${count}`);
		if (count == 5) {
			finished();
			return;
		}
	}

	function finished() {
		console.log('You "won"');
		modalStore.trigger(winModal);
	}

	function reset() {
		reasons = getReasons();
		reasons = reasons.sort(() => 0.5 - Math.random()).slice(0, 25);
	}
</script>

<div class="m-4 container mx-auto flex grow justify-center">
	<div class="grid grid-cols-5 grid-rows-5 w-full flex mx-auto grow justify-center gap-2">
		{#each reasons as r, i}
			<!-- <button class="card p-4 flex grow justify-center" on:click={handleClick}>{r.text}</button> -->
			<GridEntry
				text={r.text}
				clicked={r.clicked}
				on:click={(e) => {
					update(e, i);
				}}
			/>
		{/each}
	</div>
</div>
