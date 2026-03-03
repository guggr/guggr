/**
 * Returns whether the JWT token seems to be valid.
 *
 * Checks:
 * - correct base syntax
 * - token is not expired
 */
export function isValid(token: string) {
	const parts = token.split('.');

	if (parts.length !== 3) return false;

	let expiration;

	try {
		const claims = JSON.parse(atob(parts[1]));

		expiration = claims.exp;
	} catch {
		return false;
	}

	/** Current time in seconds since the UNIX epoch */
	const now = new Date().valueOf() / 1000;

	if (now > expiration) return false;

	return true;
}
