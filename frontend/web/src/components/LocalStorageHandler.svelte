<!--
	@component
	Client-only component which persists global stores to localStorage
	and reads their values on load.
 -->

<script lang="ts">
	import auth, { authLoaded } from '@/stores/auth.svelte';
	import settings from '@/stores/settings.svelte';
	import { onMount } from 'svelte';
	import type { Writable } from 'svelte/store';

	const localStorageStores = [
		{
			key: 'settings',
			store: settings,
		},
		{
			key: 'auth',
			store: auth,
		},
	];

	onMount(() => {
		localStorageStores.forEach(x => configureStore(x));

		authLoaded();
	});

	const configureStore = (store: { key: string; store: Writable<any> }) => {
		// set initial saved state
		const dataString = localStorage.getItem(store.key);

		if (dataString) store.store.set(JSON.parse(dataString));

		// update localStorage on every store update
		store.store.subscribe(x => {
			localStorage.setItem(store.key, JSON.stringify(x));
		});
	};
</script>
