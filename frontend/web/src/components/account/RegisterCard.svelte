<script lang="ts">
	import { config, UsersApi } from '@/api';
	import FormCard from '@/components/shared/FormCard.svelte';
	import alerts from '@/stores/alerts.svelte';

	let email = $state(''),
		name = $state(''),
		password = $state(''),
		passwordConfirm = $state('');

	const register = async () => {
		if (password !== passwordConfirm) return alerts.push('Passwords are not equal.', 'ERROR');

		const api = new UsersApi(config);

		const user = await api
			.createUser({ createUser: { email, name, password } })
			.catch(() => alerts.push('Failed to create user', 'ERROR'));

		if (!user) return;

		window.location.replace(`/login`);
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

	<label>
		<span class="label my-1">Name</span>
		<input type="text" bind:value={name} required class="input validator" placeholder="Name" />
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
