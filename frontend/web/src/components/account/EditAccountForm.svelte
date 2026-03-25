<script lang="ts">
	import CopyMyUserId from '@/components/shared/CopyMyUserId.svelte';
	import { preventDefault } from '@/lib/event';
	import auth, { authLoading } from '@/stores/auth.svelte';
	import { onMount } from 'svelte';

	let name = $state(''),
		email = $state('');

	onMount(async () => {
		await authLoading;

		name = $auth?.user.name || '';
		email = $auth?.user.email || '';
	});

	function edit() {
		//
	}
</script>

<form onsubmit={preventDefault(edit)} class="py-4 *:not-last:mb-2">
	<label class="input">
		<span class="label">Name</span>
		<input type="text" bind:value={name} placeholder="Username" />
	</label>

	<label class="input bg-base-100">
		<span class="label">Email</span>
		<input type="email" bind:value={email} placeholder="Email" disabled />
	</label>

	<CopyMyUserId />

	<div class="pt-2">
		<button type="submit" disabled class="btn btn-primary btn-soft">Update profile</button>
	</div>
</form>
