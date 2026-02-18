<script lang="ts">
	import FormCard from '@/components/shared/FormCard.svelte';

	let email = $state(''),
		password = $state(''),
		passwordConfirm = $state('');

	const register = () => {
		// TODO
	};
</script>

<FormCard title="Register" actionTitle="Register Now!" onsubmit={register}>
	<label>
		<span class="label my-1">E-Mail</span>
		<input
			type="email"
			bind:value={email}
			required
			class="input validator"
			placeholder="Email"
		/>
	</label>

	<label class="my-1">
		<span class="label my-1">Password</span>
		<input
			type="password"
			bind:value={password}
			required
			class="input validator"
			placeholder="Password"
		/>
	</label>

	<label class="my-1">
		<span class="label my-1">Confirm Password</span>
		<input
			type="password"
			bind:value={passwordConfirm}
			required
			class="input validator"
			placeholder="Password"
		/>
	</label>

	{#snippet subtext()}
		Already have an Account? <a href="/login" class="link">Sign in instead!</a>
	{/snippet}
</FormCard>
