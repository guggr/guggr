<script lang="ts">
	import { config, GroupsApi, type DisplayGroup, type DisplayGroupMember } from '@/api';
	import { preventDefault } from '@/lib/event';
	import alerts from '@/stores/alerts.svelte';
	import auth from '@/stores/auth.svelte';
	import { UserIcon, UserXIcon } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let {
		group,
		updateGroup = () => {},
	}: { group: DisplayGroup; updateGroup?: (group: DisplayGroup) => void } = $props();

	let name = $state(''),
		members = $state<DisplayGroupMember[]>([]),
		newUserId = $state('');

	let disabled = $derived(
		!['owner', 'admin'].includes(group.members.find(x => x.id === $auth?.user.id)?.role || ''),
	);

	const edit = async () => {
		const api = new GroupsApi(config);

		const newMember = {
			id: newUserId,
			name: '',
			role: 'user',
		} satisfies DisplayGroupMember;

		const newMembers = newUserId ? [...members, newMember] : members;

		const updatedGroup = await api
			.updateGroup({
				id: group.id,
				updateRequestGroup: {
					name,
					members: newMembers,
				},
			})
			.catch(() => alerts.push('Failed to update group', 'ERROR'));

		if (!updatedGroup) return;

		updateGroup(updatedGroup);
		group = updatedGroup;
		setValues();
	};

	const removeMemberCreator = (member: DisplayGroupMember) => () => {
		members = members.filter(x => x.id !== member.id);
	};

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
			<input type="text" bind:value={name} {disabled} placeholder="Group name" />
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
			<!-- nesting multiple `.list`s doesn't remove the last child's border-b -->
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

					<select
						bind:value={member.role}
						{disabled}
						class="select select-ghost select-sm"
					>
						<option value="owner">Owner</option>
						<option value="admin">Admin</option>
						<option value="user">Member</option>
					</select>
				</label>

				<button
					onclick={removeMemberCreator(member)}
					disabled={disabled || member.id === $auth?.user.id}
					class="btn btn-soft btn-error btn-square btn-sm"
				>
					<span class="sr-only">Remove member</span>
					<UserXIcon size="20" />
				</button>
			</li>
		{/snippet}
	</fieldset>

	<fieldset class="fieldset">
		<legend class="fieldset-legend">Add Member</legend>

		<label class="input">
			<span class="label">ID</span>
			<input type="text" bind:value={newUserId} {disabled} placeholder="User ID" />
		</label>
	</fieldset>

	<div class="flex flex-row-reverse gap-2 pt-2">
		<button type="submit" {disabled} class="btn btn-primary btn-soft">Save changes</button>
		<button type="button" onclick={setValues} {disabled} class="btn btn-ghost">Reset</button>
	</div>
</form>
