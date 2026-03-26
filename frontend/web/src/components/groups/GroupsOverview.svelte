<script lang="ts">
	import { config, GroupsApi, type DisplayGroup } from '@/api';
	import GroupList from '@/components/groups/GroupList.svelte';
	import CopyMyUserId from '@/components/shared/CopyMyUserId.svelte';
	import Error from '@/components/shared/Error.svelte';
	import Loading from '@/components/shared/Loading.svelte';
	import alerts from '@/stores/alerts.svelte';
	import { onMount } from 'svelte';

	let groupsPromise = $state(new Promise<DisplayGroup[]>(() => {}));

	onMount(() => {
		const api = new GroupsApi(config);

		groupsPromise = api.listGroups();
	});

	const createNewGroup = async () => {
		const api = new GroupsApi(config);

		const group = await api
			.createGroup({ createGroup: { name: 'New group' } })
			.catch(() => alerts.push('Failed to create new group', 'ERROR'));

		if (!group) return;

		const groups = await groupsPromise;

		groupsPromise = new Promise(res => {
			res([...groups, group]);
		});
	};
</script>

<div class="flex items-baseline justify-between p-4">
	<h1 class="text-lg font-bold sm:text-xl">My Groups</h1>

	<button onclick={createNewGroup} class="btn btn-primary btn-soft btn-sm sm:btn-md">
		Create new
	</button>
</div>

<div class="card bg-base-100 card-sm mb-4 w-96 max-w-full shadow-sm">
	<div class="card-body">
		<CopyMyUserId />
	</div>
</div>

{#await groupsPromise}
	<Loading />
{:then groups}
	<GroupList {groups} />
{:catch}
	<Error />
{/await}
