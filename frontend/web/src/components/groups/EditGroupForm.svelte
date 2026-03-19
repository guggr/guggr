<script lang="ts">
	import type { DisplayGroup, DisplayGroupMember } from '@/api';
	import { preventDefault } from '@/lib/event';
	import { UserIcon, UserXIcon } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let { group }: { group: DisplayGroup } = $props();

	let name = $state(''),
		members = $state<DisplayGroupMember[]>([]),
		newUserId = $state('');

	function edit() {
		//
	}

	const setValues = () => {
		name = group.name;
		members = group.members;
	};

	onMount(() => {
		setValues();
	});
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
			{#each members as member (member.id)}
				{@render groupMember(member)}
			{/each}
		</ul>

		{#snippet groupMember(member: DisplayGroupMember)}
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
						{member.name}
					</div>
				</div>

				<label>
					<span class="sr-only">Role:</span>

					<select bind:value={member.role} class="select select-ghost select-sm">
						<option value="owner">Owner</option>
						<option value="admin">Admin</option>
						<option value="user">Member</option>
					</select>
				</label>

				<button disabled class="btn btn-soft btn-error btn-square btn-sm">
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
		<button type="submit" disabled class="btn btn-primary btn-soft">Save changes</button>
		<button onclick={setValues} class="btn btn-ghost">Reset</button>
	</div>
</form>
