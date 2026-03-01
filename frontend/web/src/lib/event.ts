/** Substitute for the `preventDefault` event modifier */
export function preventDefault(
	fn: (event: Event, ...args: Array<unknown>) => void,
): (event: Event, ...args: unknown[]) => void {
	return function (...args) {
		var event = args[0];
		event.preventDefault();
		// @ts-expect-error
		return fn?.apply(this, args);
	};
}
