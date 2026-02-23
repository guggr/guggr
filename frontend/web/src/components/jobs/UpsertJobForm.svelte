<script lang="ts">
	import { preventDefault } from '@/lib/event';

	let { edit = false }: { edit?: boolean } = $props();

	let name = $state(''),
		type = $state(''),
		interval = $state(300),
		group = $state(''),
		notifications = $state(false),
		notificationText = $state('');

	const upsertJob = () => {
		// TODO
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

	<fieldset class="fieldset bg-base-100 rounded-box p-4">
		<legend>Group</legend>

		<div class="flex flex-wrap gap-2">
			<input
				type="radio"
				name="group"
				bind:group
				value="group-id"
				required
				aria-label="group name"
				class="btn btn-outline btn-sm rounded-badge"
			/>
		</div>
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
			<input
				type="text"
				bind:value={notificationText}
				required
				class="input validator"
				placeholder="Name"
			/>
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
