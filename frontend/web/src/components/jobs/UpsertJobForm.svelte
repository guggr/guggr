<script lang="ts">
	import { config, GroupsApi, JobsApi, type CreateJobDetails, type DisplayGroup } from '@/api';
	import Error from '@/components/shared/Error.svelte';
	import Loading from '@/components/shared/Loading.svelte';
	import { preventDefault } from '@/lib/event';
	import alerts from '@/stores/alerts.svelte';
	import { onMount } from 'svelte';

	let { edit = false }: { edit?: boolean } = $props();

	let id = $state('');

	let name = $state(''),
		type = $state(''),
		interval = $state(300),
		group = $state(''),
		notifications = $state(false),
		notificationText = $state(''),
		pingDetails = $state({ host: '' }),
		httpDetails = $state({ url: '' });

	let groupsPromise = $state(new Promise<DisplayGroup[]>(() => {}));

	onMount(() => {
		id = new URLSearchParams(window.location.search).get('id') ?? '';

		if (edit) loadEditJob();

		const api = new GroupsApi(config);

		groupsPromise = api.listGroups();
	});

	const loadEditJob = async () => {
		if (!id) return alerts.push('Job ID missing', 'ERROR');

		const api = new JobsApi(config);

		const job = await api
			.getJob({ id })
			.catch(() => alerts.push('Failed to fetch job', 'ERROR'));

		if (!job) return;

		name = job.name;
		type = job.jobTypeId;
		interval = job.runEvery;
		group = job.groupId;
		notifications = job.notifyUsers;
		notificationText = job.customNotification || '';

		if (job.jobTypeId === 'ping' && typeof job.details !== 'string' && 'ping' in job.details)
			pingDetails = job.details.ping;

		if (job.jobTypeId === 'http' && typeof job.details !== 'string' && 'http' in job.details)
			httpDetails = job.details.http;
	};

	const upsertJob = async () => {
		if (edit) return editJob();

		return createJob();
	};

	const editJob = async () => {
		const api = new JobsApi(config);

		let details: CreateJobDetails = { ping: pingDetails };
		if (type === 'http') details = { http: httpDetails };

		const job = await api
			.updateJob({
				id,
				updateRequestJob: {
					name,
					jobTypeId: type,
					groupId: group,
					notifyUsers: notifications,
					customNotification: notificationText,
					runEvery: interval,
					details: details,
				},
			})
			.catch(() => alerts.push('Failed to update job', 'ERROR'));

		if (!job) return;

		window.location.replace(`/jobs/details?id=${job.id}`);
	};

	const createJob = async () => {
		const api = new JobsApi(config);

		let details: CreateJobDetails = { ping: pingDetails };
		if (type === 'http') details = { http: httpDetails };

		const job = await api
			.createJob({
				createJob: {
					name,
					jobTypeId: type,
					groupId: group,
					notifyUsers: notifications,
					customNotification: notificationText,
					runEvery: interval,
					details: details,
				},
			})
			.catch(() => alerts.push('Failed to create job', 'ERROR'));

		if (!job) return;

		window.location.replace(`/jobs/details?id=${job.id}`);
	};
</script>

<form onsubmit={preventDefault(upsertJob)} class="mx-auto w-xl max-w-full *:not-last:mb-4">
	<fieldset class="fieldset bg-base-100 rounded-box p-4">
		<legend>Job Details</legend>

		<div class="flex justify-between">
			<label class="input">
				<span class="label">Job name</span>
				<input type="text" bind:value={name} required placeholder="Name" />
			</label>

			<div>
				<span class="sr-only">Job type</span>
				<div class="join">
					<input
						type="radio"
						value="http"
						bind:group={type}
						aria-label="HTTP"
						class="join-item btn"
					/>
					<input
						type="radio"
						value="ping"
						bind:group={type}
						aria-label="Ping"
						class="join-item btn"
					/>
				</div>
			</div>
		</div>

		<label class="text-base-content/80 pt-4">
			<span>Run every</span>
			<input
				type="number"
				bind:value={interval}
				class="input input-sm mx-2 w-20"
				required
				placeholder="Run interval"
			/>
			<span>seconds</span>
		</label>
	</fieldset>

	{#if type === 'ping'}
		<fieldset class="fieldset bg-base-100 rounded-box p-4">
			<legend>Ping Job Settings</legend>

			<label class="input">
				<span class="label">Host</span>
				<input type="text" bind:value={pingDetails.host} required placeholder="0.0.0.0" />
			</label>
		</fieldset>
	{/if}

	{#if type === 'http'}
		<fieldset class="fieldset bg-base-100 rounded-box p-4">
			<legend>HTTP Job Settings</legend>

			<label>
				<div class="input validator">
					<span class="label">URL</span>
					<input
						type="text"
						bind:value={httpDetails.url}
						required
						pattern={'https?\:\/\/.+'}
						placeholder="https://gug.gr"
					/>
				</div>
				<span class="label my-1 items-baseline">
					Remember to include the protocol <code>https://</code>
				</span>
			</label>
		</fieldset>
	{/if}

	<fieldset class="fieldset bg-base-100 rounded-box p-4">
		<legend>Group</legend>

		{#await groupsPromise}
			<Loading />
		{:then groups}
			<div class="flex flex-wrap gap-2">
				{#each groups as g}
					<input
						type="radio"
						name="group"
						bind:group
						value={g.id}
						required
						aria-label={g.name}
						class="btn btn-outline btn-sm rounded-badge"
					/>
				{/each}
			</div>
		{:catch}
			<Error />
		{/await}
	</fieldset>

	<fieldset class="fieldset bg-base-100 rounded-box p-4">
		<legend>Notifications</legend>

		<label>
			Enable notifications
			<input
				type="checkbox"
				bind:checked={notifications}
				class="toggle toggle-primary mx-2"
			/>
		</label>

		<label class="mt-4 w-xs max-w-full">
			<span class="label my-1"
				>Custom notification text
				<span class="badge badge-xs badge-soft">Optional</span>
			</span>
			<input type="text" bind:value={notificationText} class="input" placeholder="Name" />
			<span class="label my-1">Only takes effect if notifications are enabled</span>
		</label>
	</fieldset>

	<div class="flex flex-row-reverse gap-2">
		<button type="submit" class="btn btn-soft btn-primary">
			{edit ? 'Save job' : 'Create job'}
		</button>
		<a href="/jobs" class="btn btn-ghost">Back to jobs</a>
	</div>
</form>
