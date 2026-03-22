<script lang="ts">
	import auth from '@/stores/auth.svelte';

	let copyPromise = $state<Promise<void>>(new Promise(() => {}));

	const copyUserId = () => {
		const id = $auth?.user.id;

		if (!id) return;

		copyPromise = navigator.clipboard.writeText(id);

		setTimeout(() => (copyPromise = new Promise(() => {})), 2000);
	};
</script>

<div class="flex items-center justify-between gap-2">
	<div class="flex flex-col">
		<span class="text-base-content/80 text-xs">My User-ID:</span>
		<span class="font-mono text-base font-bold wrap-anywhere">{$auth?.user.id}</span>
	</div>

	<button onclick={copyUserId} class="btn btn-soft btn-primary btn-sm">
		{#await copyPromise}
			Copy
		{:then}
			Copied
		{:catch}
			Failed
		{/await}
	</button>
</div>
