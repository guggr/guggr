export const dateTime = new Intl.DateTimeFormat(undefined, {
	dateStyle: 'short',
	timeStyle: 'short',
});

export const relativeTime = new Intl.RelativeTimeFormat(undefined, {
	numeric: 'auto',
});

//@ts-expect-error included in es2025 target, see https://github.com/microsoft/TypeScript/pull/63046
export const duration = new Intl.DurationFormat(undefined, {
	style: 'short',
});
