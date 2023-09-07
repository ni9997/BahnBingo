<script lang="ts">
	import Footer from '$lib/components/Footer/Footer.svelte';
	import Header from '$lib/components/Header/Header.svelte';
	import Navigation from '$lib/components/Navigation/Navigation.svelte';

	import '../app.postcss';

	import { AppShell, Modal, Toast } from '@skeletonlabs/skeleton';
	import { initializeStores, Drawer, getDrawerStore } from '@skeletonlabs/skeleton';

	initializeStores();
	const drawerStore = getDrawerStore();

	function drawerOpen(): void {
		drawerStore.open({ width: 'md:w-96' });
	}

	function drawerClose(): void {
		drawerStore.close();
	}
</script>

<svelte:head>
	<title>Train Games</title>
</svelte:head>

<Modal />

<Toast />

<Drawer>
	<h2 class="p-4">Navigation</h2>
	<hr />
	<Navigation on:click={drawerClose} />
</Drawer>

<AppShell slotSidebarLeft="bg-surface-500/5 w-0 lg:w-64">
	<svelte:fragment slot="header">
		<Header on:click={drawerOpen} />
	</svelte:fragment>

	<svelte:fragment slot="sidebarLeft">
		<Navigation />
	</svelte:fragment>

	<slot />

	<svelte:fragment slot="pageFooter">
		<Footer />
	</svelte:fragment>
</AppShell>
