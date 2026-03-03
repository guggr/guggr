import type { DisplayUser, TokenResponse } from '@/api';
import { writable } from 'svelte/store';

const auth = writable<{
	auth: TokenResponse;
	user: DisplayUser;
} | null>(null);

export default auth;
