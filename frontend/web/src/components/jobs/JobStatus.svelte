<script lang="ts">
	let { online }: { online: boolean } = $props();
</script>

<div class="inline-grid *:[grid-area:1/1]">
	<div
		class:status-success={online}
		class:status-error={!online}
		class="status status-xl animate-ping motion-reduce:hidden"
	></div>
	<div class:status-success={online} class:status-error={!online} class="status status-xl"></div>
</div>
<span class:text-success={online} class:text-error={!online}>
	{online ? 'Online' : 'Offline'}
</span>
