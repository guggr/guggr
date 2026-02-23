<script lang="ts">
	import { preventDefault } from '@/lib/event';

	let name = $state(''),
		email = $state('');

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

	<div class="pt-2">
		<button type="submit" class="btn btn-primary btn-soft">Update profile</button>
	</div>
</form>
