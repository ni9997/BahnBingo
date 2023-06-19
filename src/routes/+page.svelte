<script lang="ts">
	// import Grid from
	let reasons = []

	for (let index = 0; index < 25; index++) {
		reasons.push({id: index, text: `Reason ${index}`})
	}

</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="grid grid-cols-5 grid-rows-5 h-full w-full mx-auto flex justify-center">
		{#each reasons as {id, text}, index(id)}
			<div class="card p-4 flex justify-center">text</div>
		{/each}
	</div>
</div>
