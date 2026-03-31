import { config, GroupsApi, type DisplayGroup } from '@/api';

/**
 * Caching for getGroup requests.
 */
export class GroupsCache {
	private cache = new Map<string, Promise<DisplayGroup>>();
	private groupsApi = new GroupsApi(config);

	/**
	 * Checks cache for group data with given id or requests it and stores it in the cache.
	 * @param id Group id to request
	 * @returns DisplayGroup data for given id
	 */
	async get(id: string): Promise<DisplayGroup> {
		if (!this.cache.has(id)) {
			this.cache.set(id, this.groupsApi.getGroup({ id }));
		}

		// undefined is checked above
		return this.cache.get(id) as Promise<DisplayGroup>;
	}
}
