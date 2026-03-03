<script lang="ts">
	import { config, GroupsApi } from '@/api';
	import alerts from '@/stores/alerts.svelte';

	const createNewGroup = async () => {
		const api = new GroupsApi(config);

		const group = await api
			.createGroup({ createGroup: { name: 'New group' } })
			.catch(() => alerts.push('Failed to create new group', 'ERROR'));

		if (!group) return;

		// TODO update groups
	};
</script>

<button onclick={createNewGroup} class="btn btn-primary btn-soft btn-sm sm:btn-md">
	Create new
</button>
