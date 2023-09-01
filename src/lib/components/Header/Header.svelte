<script lang="ts">
	import { AppBar, LightSwitch } from '@skeletonlabs/skeleton';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead"><img class="max-h-6" src="logo.svg" alt="Logo" /></svelte:fragment>
	Bahn Bingo
	<svelte:fragment slot="trail"><LightSwitch /></svelte:fragment>
</AppBar>
