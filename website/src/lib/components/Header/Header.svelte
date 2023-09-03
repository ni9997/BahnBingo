<script lang="ts">
	import { AppBar, LightSwitch } from '@skeletonlabs/skeleton';
	import IconMenu2 from '@tabler/icons-svelte/dist/svelte/icons/IconMenu2.svelte';
</script>

<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
	<svelte:fragment slot="lead">
		<div class="flex items-center">
			<button class="lg:hidden btn btn-sm mr-4" on:click>
				<IconMenu2 />
			</button>
			<!-- <strong class="text-xl uppercase">Skeleton</strong> -->
		</div>
	</svelte:fragment>

	Train Games
	<svelte:fragment slot="trail"><LightSwitch /></svelte:fragment>
</AppBar>
