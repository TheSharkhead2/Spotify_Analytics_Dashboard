import { writable, type Writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

async function authenticated(): Promise<boolean> {
    return invoke("authenticated").then((result) => {return result as boolean}).catch((err) => {return false});
}

console.log(await authenticated());
export const AUTHENTICATED = writable(await authenticated());

export const STYLE = {
    data_point_color: '#0798f8',
    data_point_r: 3
};

export const PLAYLIST_DISPLAY: Writable<boolean> = writable(false); // playlist submenu being displayed (true false)
export const PLAYLIST_DISPLAY_ID: Writable<string> = writable(''); // current playlist submenu being displayed (playlist id)