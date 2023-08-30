<script lang="ts">
	// Most of your app wide CSS should be put in this file
	import '../app.postcss';

	import { AppBar } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import { initializeStores } from '@skeletonlabs/skeleton';

	initializeStores();
</script>

<div class="h-screen flex-row">
	<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
		<svelte:fragment slot="lead">(icon)</svelte:fragment>
		Bahn Bingo
		<svelte:fragment slot="trail"><LightSwitch /></svelte:fragment>
	</AppBar>

	<slot />
</div>
