<!--
	@component
	Shows a centered error message
 -->

<script lang="ts">
	import { SirenIcon } from '@lucide/svelte';
</script>

<div class="@container flex w-full grow flex-col items-center justify-center gap-4 p-2">
	<SirenIcon class="text-error size-12 @min-sm:size-16" />
	<div class="w-52 max-w-full text-center font-bold @min-sm:text-lg">Something went wrong.</div>
</div>
