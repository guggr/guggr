<script lang="ts">
	import { AuthApi, config } from '@/api';
	import FormCard from '@/components/shared/FormCard.svelte';
	import alerts from '@/stores/alerts.svelte';
	import auth from '@/stores/auth.svelte';

	let email = $state(''),
		password = $state('');

	const login = async () => {
		const api = new AuthApi(config);

		const loginResponse = await api
			.authLogin({
				loginRequest: {
					email,
					password,
				},
			})
			.catch(() => alerts.push('Login failed', 'ERROR'));

		if (!loginResponse) return;

		auth.set(loginResponse);

		window.location.replace(`/jobs`);
	};
</script>

<FormCard title="Login" actionTitle="Login" onsubmit={login}>
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

	{#snippet subtext()}
		New to guggr? <a href="/register" class="link">Create an account!</a>
	{/snippet}
</FormCard>
