<script lang="ts">
	import FormCard from '@/components/shared/FormCard.svelte';

	let email = $state(''),
		password = $state('');

	const login = () => {
		// TODO
	};
</script>

<FormCard title="Login" actionTitle="Login" onsubmit={login} {subtext}>
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
</FormCard>

{#snippet subtext()}
	New to guggr? <a href="/register" class="link">Create an account!</a>
{/snippet}
