<script lang="ts">
	import { preventDefault } from '@/lib/event';
	import { UserIcon, UserXIcon } from '@lucide/svelte';

	let name = $state(''),
		newUserId = $state('');

	function edit() {
		//
	}

	function reset() {
		//
	}
</script>

<form onsubmit={preventDefault(edit)} class="w-md max-w-full">
	<fieldset class="fieldset">
		<legend class="sr-only">Group Settings</legend>

		<label class="input">
			<span class="label">Name</span>
			<input type="text" bind:value={name} placeholder="Group name" />
		</label>
	</fieldset>

	<fieldset class="fieldset">
		<legend class=" fieldset-legend">Group Members</legend>

		<ul class="list @container">
			{@render groupMember()}
			{@render groupMember()}
		</ul>

		{#snippet groupMember()}
			<!-- nesting multiple `.list`s doesn't remove the last childs border-b -->
			<li
				class="list-row items-center last:after:hidden @max-xs:grid-cols-[1fr] @max-xs:gap-2"
			>
				<div class="@max-xs:hidden">
					<UserIcon class="text-primary/70" />
				</div>

				<div class="min-w-0">
					<div class="text-base-content/80 truncate text-sm font-semibold">
						<span class="sr-only">User:</span>
						Username
					</div>
				</div>

				<label>
					<span class="sr-only">Role:</span>

					<select class="select select-ghost select-sm">
						<option>Owner</option>
						<option>Member</option>
					</select>
				</label>

				<button class="btn btn-soft btn-error btn-square btn-sm">
					<span class="sr-only">Kick user</span>
					<UserXIcon size="20" />
				</button>
			</li>
		{/snippet}
	</fieldset>

	<fieldset class="fieldset">
		<legend class="fieldset-legend">Add Member</legend>

		<label class="input">
			<span class="label">ID</span>
			<input type="text" bind:value={newUserId} placeholder="User ID" />
		</label>
	</fieldset>

	<div class="flex flex-row-reverse gap-2 pt-2">
		<button type="submit" class="btn btn-primary btn-soft">Save changes</button>
		<button onclick={reset} class="btn btn-ghost">Reset</button>
	</div>
</form>
