<script lang="ts">
	import alerts from '@/stores/alerts.svelte';
	import { InfoIcon, SirenIcon, XIcon } from '@lucide/svelte';

	const remove = (id: string) => () => alerts.remove(id);
</script>

<ul class="toast pointer-events-none z-50 w-11/12 max-w-xl items-end">
	{#each $alerts as alert (alert.id)}
		<li role="alert" class="card card-sm card-border bg-base-100 pointer-events-auto shadow-lg">
			<div class="card-body flex-row items-center gap-3">
				{#if alert.level === 'INFO'}
					<InfoIcon class="text-info shrink-0" />
					<span class="sr-only">Info:</span>
				{:else}
					<SirenIcon class="text-error shrink-0" />
					<span class="sr-only">Error:</span>
				{/if}

				<span class="grow overflow-hidden text-ellipsis">{alert.msg}</span>

				<button onclick={remove(alert.id)} class="btn btn-square btn-sm btn-ghost">
					<span class="sr-only">Close alert</span>
					<XIcon size="20" />
				</button>
			</div>
		</li>
	{/each}
</ul>
