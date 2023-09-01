<script lang="ts">
    import IconTrain from '@tabler/icons-svelte/dist/svelte/icons/IconTrain.svelte';
    import IconHeart from '@tabler/icons-svelte/dist/svelte/icons/IconHeart.svelte'
</script>

<div class="mx-4">
	<hr class="" />
	<div class="my-2">
		<div class="flex justify-center items-center gap-2">
			<h4>Made with</h4>
			<IconHeart class="stroke-red-600" />
			<h4>while waiting for</h4>
			<!-- <IconCoffee /> -->
			<IconTrain />
		</div>
	</div>
</div>
