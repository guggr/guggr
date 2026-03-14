<script lang="ts">
	import { config, JobsApi, type DisplayJob, type DisplayJobRun } from '@/api';
	import JobRuns from '@/components/jobs/details/JobRuns.svelte';
	import Error from '@/components/shared/Error.svelte';
	import Loading from '@/components/shared/Loading.svelte';
	import { relativeTime } from '@/lib/formatter';
	import { getJobName } from '@/lib/jobs';
	import alerts from '@/stores/alerts.svelte';
	import { ActivityIcon, PenIcon, Trash2Icon } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let id = $state(''),
		jobName = $state('');

	let jobPromise = $state(new Promise<DisplayJob>(() => {})),
		jobRunsPromise = $state(new Promise<DisplayJobRun[]>(() => {}));

	onMount(async () => {
		id = new URLSearchParams(window.location.search).get('id') ?? '';

		const api = new JobsApi(config);

		jobPromise = api.getJob({ id });
		jobRunsPromise = api.listJobRuns({ id });

		jobName = (await jobPromise).name;
	});

	const deleteJob = async () => {
		if (!confirm(`Do you really want to delete the Job "${jobName}"?`)) return;

		const api = new JobsApi(config);

		api.deleteJob({ id })
			.then(() => window.location.replace('/jobs'))
			.catch(() => alerts.push('Failed to delete the job', 'ERROR'));
	};
</script>

<svelte:head>
	<title>Job {jobName} Details | guggr</title>
</svelte:head>

<div class="@container mb-4 flex items-baseline justify-between gap-2">
	<div class="breadcrumbs px-2 text-sm">
		<menu>
			<li><a href="/">Home</a></li>
			<li><a href="/jobs">Jobs</a></li>
			<li><a href={`/jobs/details?id=${id}`}>{jobName || 'Job'}</a></li>
		</menu>
	</div>

	<div class="flex flex-row-reverse gap-2">
		<a href={`/jobs/edit?id=${id}`} class="btn btn-soft btn-sm">
			<PenIcon size="16" /> <span class="@max-md:sr-only">Edit job</span>
		</a>

		<button onclick={deleteJob} class="btn btn-soft btn-error btn-sm">
			<Trash2Icon size="16" /> <span class="@max-md:sr-only">Delete job</span>
		</button>
	</div>
</div>

<div class="card card-side bg-base-100 shadow-md">
	{#await jobPromise}
		<Loading />
	{:then job}
		<figure class="text-primary/60 hidden p-6 md:block">
			<ActivityIcon size="48" />
		</figure>

		<div class="card-body flex flex-col justify-between gap-2 sm:flex-row sm:items-center">
			<div>
				<div
					class="card-title text-md text-base-content/90 sm:text-base-content sm:text-2xl"
				>
					<h2 class="truncate">
						<span class="sr-only">Job name:</span>
						{job.name}
					</h2>
					<span class="badge badge-primary badge-soft badge-sm whitespace-nowrap">
						{getJobName(job.jobTypeId)} Job
					</span>
				</div>

				<div class="text-base-content/80">
					<span class="sr-only">Group: </span>
					<!-- TODO group name -->
					<a href="/groups" class="link link-hover">{job.groupId}</a>
				</div>
			</div>

			<div class="stats hidden sm:inline-grid">
				{@render statusStat(job)}
			</div>
		</div>
	{:catch}
		<Error />
	{/await}
</div>

<div class="stats rounded-box bg-base-100 mt-4 w-full py-4 shadow-md sm:hidden">
	{#await jobPromise}
		<Loading />
	{:then job}
		{@render statusStat(job)}
	{:catch}
		<Error />
	{/await}
</div>

{#snippet statusStat(job: DisplayJob)}
	{@const lastScheduledDiffMinutes = Math.round(
		(Date.now() - (job.lastScheduled?.valueOf() || 0)) / 1000 / 60,
	)}

	<div class="stat py-0">
		<div class="stat-title">Current Status</div>
		<div class="stat-value text-success flex items-center gap-2 text-3xl">
			<div class="inline-grid *:[grid-area:1/1]">
				<div
					class="status status-success status-xl animate-ping motion-reduce:hidden"
				></div>
				<div class="status status-success status-xl"></div>
			</div>
			Online
		</div>
		<div class="stat-desc">
			{#if job.lastScheduled}
				Last checked {relativeTime.format(-lastScheduledDiffMinutes, 'minutes')}
			{:else}
				The job hasn't been run yet
			{/if}
		</div>
	</div>
{/snippet}

<div class="bg-base-100 rounded-box my-4 p-4 shadow-md">
	<h2 class="text-base-content/80 mb-2 text-lg font-bold">Timeline</h2>

	<JobRuns {jobRunsPromise} />
</div>
