<script lang="ts">
	import { config, GroupsApi, JobsApi, type DisplayJob } from '@/api';
	import JobStatus from '@/components/jobs/JobStatus.svelte';
	import Error from '@/components/shared/Error.svelte';
	import Loading from '@/components/shared/Loading.svelte';
	import { duration, relativeTime } from '@/lib/formatter';
	import { getJobName } from '@/lib/jobs';
	import { ActivityIcon, ChevronRightIcon } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let jobsPromise = $state(new Promise<DisplayJob[]>(() => {}));

	let filterOfflineOnly = $state(false);

	const groupsApi = new GroupsApi(config);

	onMount(() => {
		const api = new JobsApi(config);

		jobsPromise = api.listJob();
	});
</script>

<div class="mb-8 flex flex-row-reverse items-baseline justify-between gap-2">
	<a href="/jobs/create" class="btn btn-primary btn-soft">Create new job</a>

	<label class="label text-sm select-none">
		<input type="checkbox" bind:checked={filterOfflineOnly} class="toggle toggle-primary" />
		Offline jobs only
	</label>
</div>

{#await jobsPromise}
	<Loading />
{:then jobs}
	<ul class="*:not-last:mb-6">
		{#each jobs.filter(x => !filterOfflineOnly || (filterOfflineOnly && !x.reachable)) as j}
			{@render job(j)}
		{/each}
	</ul>
{:catch}
	<Error />
{/await}

{#snippet job(j: DisplayJob)}
	{@const lastScheduledDiffMinutes = Math.round(
		(Date.now() - (j.lastScheduled?.valueOf() || 0)) / 1000 / 60,
	)}

	<li class="card card-sm sm:card-md card-side bg-base-100 shadow-md">
		<figure class="text-primary/60 hidden p-6 md:block">
			<ActivityIcon size="48" />
		</figure>

		<div class="card-body flex-row items-center justify-between">
			<div class="flex grow flex-col justify-between gap-2 sm:flex-row sm:items-center">
				<div>
					<div
						class="card-title text-md text-base-content/90 sm:text-base-content sm:text-2xl"
					>
						<h2 class="truncate">{j.name}</h2>
						<span class="badge badge-primary badge-soft badge-sm whitespace-nowrap">
							{getJobName(j.jobTypeId)} Job
						</span>
					</div>
					<ul
						class="text-base-content/80 hidden *:inline *:not-first:before:content-['•'] sm:block"
					>
						<li>
							<span class="sr-only">Group: </span>
							{#await groupsApi.getGroup({ id: j.groupId })}
								<a href="/groups" class="link link-hover">{j.groupId}</a>
							{:then group}
								<a href="/groups" class="link link-hover">{group.name}</a>
							{:catch}
								<a href="/groups" class="link link-hover">{j.groupId}</a>
							{/await}
						</li>
						<li>
							<span class="sr-only">Execution interval: </span>
							every {duration.format({
								minutes: Math.floor(j.runEvery / 60),
								seconds: j.runEvery % 60,
							})}
						</li>
					</ul>
				</div>

				<div>
					<div class="stats">
						<div class="stat px-2 py-0 sm:min-w-56 sm:px-6">
							<div class="stat-title hidden sm:block">Current Status</div>
							<div class="stat-value flex items-center gap-2 text-3xl">
								<JobStatus online={j.reachable} />
							</div>
							<div class="stat-desc hidden sm:block">
								{#if j.lastScheduled}
									Last checked {relativeTime.format(
										-lastScheduledDiffMinutes,
										'minutes',
									)}
								{:else}
									The job hasn't been run yet
								{/if}
							</div>
						</div>
					</div>
				</div>
			</div>

			<div class=" sm:hidden">
				<a href={`/jobs/details?id=${j.id}`} class="btn btn-primary btn-soft pr-2">
					Details <ChevronRightIcon size="20" />
				</a>
			</div>
		</div>

		<a
			href={`/jobs/details?id=${j.id}`}
			class="btn btn-primary btn-soft hidden h-auto p-3 [writing-mode:sideways-lr] sm:inline-flex"
		>
			Details
			<ChevronRightIcon size="20" class="-rotate-90" />
		</a>
	</li>
{/snippet}
