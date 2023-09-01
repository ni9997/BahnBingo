<script lang="ts">
	import Footer from '$lib/components/Footer/Footer.svelte';
	import Header from '$lib/components/Header/Header.svelte';

	import '../app.postcss';

	import { AppShell, Modal } from '@skeletonlabs/skeleton';
	import { initializeStores } from '@skeletonlabs/skeleton';

	initializeStores();
</script>

<svelte:head>
	<title>Bahnbingo</title>
</svelte:head>

<Modal />

<AppShell>
	<svelte:fragment slot="header">
		<Header />
	</svelte:fragment>

	<slot />

	<svelte:fragment slot="pageFooter">
		<Footer />
	</svelte:fragment>
</AppShell>
