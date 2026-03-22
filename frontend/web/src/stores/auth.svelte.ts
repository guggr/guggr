import type { DisplayUser, TokenResponse } from '@/api';
import { writable } from 'svelte/store';

const auth = writable<{
	auth: TokenResponse;
	user: DisplayUser;
} | null>(null);

export default auth;

/** Call this once auth data has been retrieved from localStorage */
export let authLoaded: (value?: unknown) => void;
/** Resolves once auth data has been read from localStorage */
export const authLoading = $state(
	new Promise(res => {
		authLoaded = res;
	}),
);
