import { writable } from 'svelte/store';

const settings = writable<{ theme?: '' | 'dark' | 'light' }>({});

export default settings;
