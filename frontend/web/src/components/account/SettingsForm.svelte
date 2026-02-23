<script lang="ts">
	import settings from '@/stores/settings.svelte';
</script>

<fieldset class="fieldset">
	<legend class="fieldset-legend">Theme</legend>

	<div class="join">
		<input
			type="radio"
			bind:group={$settings.theme}
			value=""
			aria-label="System Default"
			class="join-item btn"
		/>
		<input
			type="radio"
			bind:group={$settings.theme}
			value="light"
			aria-label="Light"
			class="join-item btn"
		/>
		<input
			type="radio"
			bind:group={$settings.theme}
			value="dark"
			aria-label="Dark"
			class="join-item btn"
		/>
	</div>
</fieldset>
