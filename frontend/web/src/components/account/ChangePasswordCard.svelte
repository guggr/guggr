<script lang="ts">
	import FormCard from '@/components/shared/FormCard.svelte';

	let oldPassword = $state(''),
		newPassword = $state(''),
		newPasswordConfirm = $state('');

	const changePassword = () => {
		// TODO
	};
</script>

<FormCard disabled title="Change password" actionTitle="Change password" onsubmit={changePassword}>
	<label class="my-1">
		<span class="label my-1">Old password</span>
		<input
			type="password"
			bind:value={oldPassword}
			required
			class="input validator"
			placeholder="Old password"
		/>
	</label>

	<label class="my-1">
		<span class="label my-1">New password</span>
		<input
			type="password"
			bind:value={newPassword}
			required
			class="input validator"
			placeholder="New password"
		/>
	</label>

	<label class="my-1">
		<span class="label my-1">Confirm new password</span>
		<input
			type="password"
			bind:value={newPasswordConfirm}
			required
			class="input validator"
			placeholder="New password"
		/>
	</label>

	{#snippet subtext()}
		Want to keep your old password? <a href="/account" class="link">Go back</a>
	{/snippet}
</FormCard>
