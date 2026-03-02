import { writable } from 'svelte/store';

type AlertLevel = 'INFO' | 'ERROR';

const alertsStore = writable<{ id: string; msg: string; level: AlertLevel }[]>([]);

const alerts = {
	subscribe: alertsStore.subscribe,

	push: (msg: string, level: AlertLevel = 'INFO') =>
		alertsStore.update(x => [...x, { id: `${Math.random() * 1000000}`, msg, level }]),
	remove: (id: string) => alertsStore.update(x => x.filter(x => x.id !== id)),
};

export default alerts;
