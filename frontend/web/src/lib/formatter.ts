export const dateTime = new Intl.DateTimeFormat(undefined, {
	dateStyle: 'short',
	timeStyle: 'short',
});

export const relativeTime = new Intl.RelativeTimeFormat(undefined, {
	numeric: 'auto',
});
