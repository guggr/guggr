import { writable } from 'svelte/store';

const SETTINGS_LOCAL_STORAGE_KEY = 'settings';

const settings = writable<{ theme?: '' | 'dark' | 'light' }>({});

// set initial saved state
setSettingsFromLocalStorage();

// update localStorage on every settings update
settings.subscribe(x => {
	if (!localStorage) return;

	localStorage.setItem(SETTINGS_LOCAL_STORAGE_KEY, JSON.stringify(x));
});

export default settings;

function setSettingsFromLocalStorage() {
	if (!localStorage) return;

	const settingsString = localStorage.getItem(SETTINGS_LOCAL_STORAGE_KEY);

	if (settingsString) settings.set(JSON.parse(settingsString));
}
