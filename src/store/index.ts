import { Store } from "tauri-plugin-store-api";
import {  appDataDir, join } from '@tauri-apps/api/path';

async function get_path(params: string): Promise<string> {
    const appDataDirPath = await appDataDir();
    const path = await join(appDataDirPath, params);
    return path
}


export async function set_value(key: string, value: any) {
    const path = await get_path('.settings.dat');
    console.log(path)
    const store = new Store(path);
    await store.set(key, value)
    await store.save()
}


export async function get_value(key: string) {
    const path = await get_path('.settings.dat');
    const store = new Store(path);
    return await store.get(key)
}
