<script lang="ts">
	import type { DisplayJobRun } from '@/api';
	import Error from '@/components/shared/Error.svelte';
	import Loading from '@/components/shared/Loading.svelte';
	import { dateTime } from '@/lib/formatter';

	let selected = $state<DisplayJobRun | null>(null);

	let { jobRunsPromise = new Promise(() => {}) }: { jobRunsPromise?: Promise<DisplayJobRun[]> } =
		$props();

	function showDetailsCreator(run: DisplayJobRun) {
		return () => {
			selected = run;
		};
	}
</script>

{#await jobRunsPromise}
	<Loading />
{:then jobRuns}
	<ul class="flex flex-row-reverse gap-1 overflow-x-auto p-1">
		{#each jobRuns as jobRun (jobRun.id)}
			{@render run(jobRun)}
		{:else}
			<li class="w-full text-base-content/80 font-bold">No job runs available yet.</li>
		{/each}
	</ul>

	{#if selected}
		<div class="mt-4 flex flex-wrap items-center gap-6">
			{#if selected.reachable}
				<p class="text-success text-2xl font-bold">Online</p>
			{:else}
				<p class="text-error text-2xl font-bold">Offline</p>
			{/if}

			<ul class="text-sm">
				{#if selected.triggeredNotification}
					<li class="text-base-content/80 italic">Notification sent</li>
				{/if}

				<li>
					<span class="text-base-content/80">Time:</span>
					<b class="font-bold">{dateTime.format(selected.timestamp)}</b>
				</li>

				{#if selected.details && 'ping' in selected.details}
					<li>
						<span class="text-base-content/80">Latency:</span>
						<b class="font-bold">{selected.details.ping.latency}</b>
					</li>
				{/if}

				{#if selected.details && 'http' in selected.details}
					<li>
						<span class="text-base-content/80">Latency:</span>
						<b class="font-bold">{selected.details.http.latency}</b>
					</li>
					<li>
						<span class="text-base-content/80">Response status code:</span>
						<b class="font-bold">{selected.details.http.statusCode}</b>
					</li>
				{/if}
			</ul>
		</div>
	{:else}
		<p class="text-base-content/80 mt-4 text-sm">
			Click on a job run to display further details.
		</p>
	{/if}
{:catch}
	<Error />
{/await}

{#snippet run(r: DisplayJobRun)}
	<li
		class={{
			'bg-success': r.reachable,
			'bg-error': !r.reachable,
			'rounded-field flex h-12 w-4 min-w-2.5 shrink': true,
		}}
		title="Time: {dateTime.format(r.timestamp)}"
	>
		<p class="sr-only">
			{#if r.reachable}
				Successful
			{:else}
				Failed
			{/if}
			at {dateTime.format(r.timestamp)}
		</p>

		<!-- TODO focus/hover style -->
		<button onclick={showDetailsCreator(r)} class="w-full cursor-pointer">
			<span class="sr-only">Show details</span>
		</button>
	</li>
{/snippet}
