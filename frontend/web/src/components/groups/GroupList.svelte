<script lang="ts">
	import EditGroupForm from '@/components/groups/EditGroupForm.svelte';
	import { UsersIcon } from '@lucide/svelte';
</script>

<ul class="list">
	{@render group()}
	{@render group()}
</ul>

{#snippet group()}
	<li class="list-row @container gap-2 p-0">
		<details class="collapse-arrow list-col-grow collapse" name="accordion-group-list">
			<summary class="collapse-title select-none">
				<div class="flex gap-2">
					<UsersIcon size="24" class="text-primary/70 m-2 shrink-0" />

					<div class="overflow-hidden">
						<div class="flex gap-1">
							<span class="truncate">Group name</span>
							<span
								class="badge badge-primary badge-soft badge-sm text-nowrap @max-xs:hidden"
							>
								Owner
							</span>
						</div>
						<div class="truncate text-xs font-semibold uppercase opacity-60">
							User 1, User 2
						</div>
					</div>
				</div>
			</summary>

			<div class="collapse-content">
				<EditGroupForm />
			</div>
		</details>
	</li>
{/snippet}
