import { writable } from 'svelte/store';
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