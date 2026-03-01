<script lang="ts">
	let selected = $state('');

	function showDetailsCreator(id: string) {
		return () => {
			selected = id;
		};
	}
</script>

<ul class="flex flex-row-reverse gap-1 overflow-x-auto p-1">
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
	{@render run()}
</ul>

{#if selected}
	<ul class="mt-4 text-sm">
		<li>
			<span class="text-base-content/80">Time:</span>
			<b class="font-bold">2026-02-31 14:85</b>
		</li>

		<li>
			<span class="text-base-content/80">Latency:</span>
			<b class="font-bold">12ms</b>
		</li>
	</ul>
{:else}
	<p class="text-base-content/80 mt-4 text-sm">Click on a job run to display further details.</p>
{/if}

{#snippet run()}
	<li
		class="bg-success rounded-field flex h-12 w-4 min-w-2.5 shrink"
		title="Time: 2026-02-31 14:85"
	>
		<p class="sr-only">Successful at 2026-02-31 14:85 with 14ms latency</p>

		<!-- TODO focus/hover style -->
		<button onclick={showDetailsCreator('id')} class="w-full cursor-pointer">
			<span class="sr-only">Show details</span>
		</button>
	</li>
{/snippet}
