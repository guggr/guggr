<!--
	@component
	Updating the theme from the settings store
 -->

<script lang="ts">
	import settings from '@/stores/settings.svelte';

	$effect(() => {
		// for the system theme to take effect, we need to explicitly remove the data-theme attribute
		if (!$settings.theme) return document.documentElement.removeAttribute('data-theme');

		document.documentElement.setAttribute('data-theme', $settings.theme || '');
	});
</script>
