import { AuthApi, Configuration } from '@/api';
import { isValid } from '@/lib/jwt';
import auth, { authLoading } from '@/stores/auth.svelte';
import { get } from 'svelte/store';

export * from '@/api/gen';

const BASE_PATH = import.meta.env.DEV ? 'http://localhost:8081' : '';

export const config = new Configuration({
	basePath: BASE_PATH,
	accessToken: async () => {
		await authLoading;

		const authData = get(auth);
		if (!authData) return '';

		let accessToken = authData.auth.accessToken;

		if (!isValid(accessToken)) {
			const refreshedTokens = await refreshAccessToken(authData.auth.refreshToken).catch(
				loginRequired,
			);

			if (!refreshedTokens) return '';

			auth.update(x => (x ? { ...x, auth: refreshedTokens } : null));
			accessToken = refreshedTokens.accessToken;
		}

		return accessToken;
	},
});

async function refreshAccessToken(refreshToken: string) {
	const authApi = new AuthApi(new Configuration({ basePath: BASE_PATH }));

	return await authApi.authRefreshToken({ tokenRefreshRequest: { refreshToken } });
}

function loginRequired() {
	auth.set(null);

	if (!window) return;

	window.location.replace('/login');
}
