<script lang="ts">
	import type { DisplayGroup } from '@/api';
	import EditGroupForm from '@/components/groups/EditGroupForm.svelte';
	import auth from '@/stores/auth.svelte';
	import { UsersIcon } from '@lucide/svelte';

	let { groups }: { groups: DisplayGroup[] } = $props();
</script>

<ul class="list">
	{#each groups as g (g.id)}
		{@render group(g)}
	{:else}
		<li class="font-bold text-base-content/70 text-center my-4">
			No groups available yet. Create your first!
		</li>
	{/each}
</ul>

{#snippet group(group: DisplayGroup)}
	<li class="list-row @container gap-2 p-0">
		<details
			class="collapse-arrow list-col-grow open:border-base-content/10 collapse border-2 border-transparent"
			name="accordion-group-list"
		>
			<summary class="collapse-title select-none">
				<div class="flex gap-2">
					<UsersIcon size="24" class="text-primary/70 m-2 shrink-0" />

					<div class="overflow-hidden">
						<div class="flex gap-1">
							<span class="truncate">{group.name}</span>
							<span
								class="badge badge-primary badge-soft badge-sm font-bold text-nowrap uppercase @max-xs:hidden"
							>
								{group.members.find(x => x.id === $auth?.user.id)?.role ||
									'Unknown role'}
							</span>
						</div>
						<div class="truncate text-xs font-semibold uppercase opacity-60">
							{group.members.map(x => x.name).join(', ')}
						</div>
					</div>
				</div>
			</summary>

			<div class="collapse-content">
				<EditGroupForm {group} />
			</div>
		</details>
	</li>
{/snippet}
