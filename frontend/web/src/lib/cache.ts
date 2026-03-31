/**
 * Implementation for a generic cache for requests.
 */
export default class Cache<T> {
	private cache = new Map<string, T>();

	constructor(private fetchOne: (id: string) => Promise<T>) {}

	/**
	 * Checks the cache for the item, and - if not present - fetches the item.
	 * @param id item id to request
	 * @returns item for given id
	 */
	async get(id: string) {
		if (this.cache.has(id)) this.cache.get(id);

		// cache miss
		let item = await this.fetchOne(id);

		if (!item) throw new Error('item fetch failed');

		this.cache.set(id, item);
		return item;
	}
}
