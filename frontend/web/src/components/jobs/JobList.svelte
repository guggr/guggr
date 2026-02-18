<script lang="ts">
	import { ActivityIcon, ChevronRightIcon } from '@lucide/svelte';
</script>

<ul class="*:not-last:mb-6">
	{@render job()}
</ul>

{#snippet job()}
	<li class="card card-sm sm:card-md card-side bg-base-100 shadow-md">
		<figure class="text-primary/60 hidden p-6 md:block">
			<ActivityIcon size="48" />
		</figure>

		<div class="card-body flex-row items-center justify-between">
			<div class="flex grow flex-col justify-between gap-2 sm:flex-row sm:items-center">
				<div>
					<div
						class="card-title text-md text-base-content/70 sm:text-base-content sm:text-2xl"
					>
						<h2 class="truncate">Job name</h2>
						<span class="badge badge-primary badge-soft badge-sm whitespace-nowrap">
							HTTP Job
						</span>
					</div>
					<ul
						class="text-base-content/80 hidden *:inline *:not-first:before:content-['•'] sm:block"
					>
						<li>
							<span class="sr-only">Group: </span>
							<a href="/groups" class="link link-hover">My cool group</a>
						</li>
						<li>
							<span class="sr-only">Execution interval: </span>
							every 3 minutes
						</li>
					</ul>
				</div>

				<div>
					<div class="stats">
						<div class="stat px-2 py-0 sm:px-6">
							<div class="stat-title hidden sm:block">Current Status</div>
							<div class="stat-value text-success flex items-center gap-2 text-3xl">
								<div class="inline-grid *:[grid-area:1/1]">
									<div
										class="status status-success status-xl animate-ping motion-reduce:hidden"
									></div>
									<div class="status status-success status-xl"></div>
								</div>
								Online
							</div>
							<div class="stat-desc hidden sm:block">Last checked 2 minutes ago</div>
						</div>
					</div>
				</div>
			</div>

			<div class=" sm:hidden">
				<a href="/jobs" class="btn btn-primary btn-soft pr-2">
					Details <ChevronRightIcon size="20" />
				</a>
			</div>
		</div>

		<a
			href="/jobs"
			class="btn btn-primary btn-soft hidden h-auto p-3 [writing-mode:sideways-lr] sm:inline-flex"
		>
			Details
			<ChevronRightIcon size="20" class="-rotate-90" />
		</a>
	</li>
{/snippet}
