<script lang="ts">
	import auth from '@/stores/auth.svelte';
	import { LogOutIcon, SettingsIcon, UserIcon, UsersIcon } from '@lucide/svelte';
</script>

{#if $auth}
	<button
		class="btn btn-ghost"
		popovertarget="popover-nav-acc"
		style="anchor-name:--anchor-nav-acc"
	>
		<UserIcon size="20" />
		{$auth.user.name}
	</button>
	<ul
		popover
		id="popover-nav-acc"
		class="dropdown dropdown-end bg-base-100 menu rounded-box menu-lg sm:menu-md mt-1 w-52 shadow"
		style="position-anchor:--anchor-nav-acc"
	>
		<li>
			<a href="/account"><SettingsIcon class="sm:size-4.5" /> Account</a>
		</li>
		<li>
			<a href="/groups"><UsersIcon class="sm:size-4.5" /> Groups</a>
		</li>
		<li>
			<button><LogOutIcon class="sm:size-4.5" /> Logout</button>
		</li>
	</ul>
{:else}
	<div class="flex flex-row-reverse gap-2">
		<a href="/register" class="btn btn-primary">Register Now!</a>
		<a href="/login" class="btn btn-ghost">Login</a>
	</div>
{/if}
