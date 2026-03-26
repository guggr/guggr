<script lang="ts">
	import { preventDefault } from '@/lib/event';
	import type { Snippet } from 'svelte';

	let {
		onsubmit,
		title,
		actionTitle,
		disabled = false,
		children,
		subtext,
	}: {
		onsubmit: () => void;
		title: string;
		actionTitle: string;
		disabled?: boolean;
		children?: Snippet;
		subtext?: Snippet;
	} = $props();
</script>

<div class="flex grow items-center justify-center">
	<form onsubmit={preventDefault(onsubmit)} class="w-xs p-2">
		<fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4">
			<legend class="fieldset-legend">{title}</legend>

			{@render children?.()}

			<button type="submit" {disabled} class="btn dark:btn-soft btn-primary mt-4"
				>{actionTitle}</button
			>
		</fieldset>

		{#if subtext}
			<p class="text-base-content/60 mt-1 text-sm">
				{@render subtext()}
			</p>
		{/if}
	</form>
</div>
