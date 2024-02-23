import { Store } from "tauri-plugin-store-api";
import { resolveResource } from "@tauri-apps/api/path";



export async function set_value(key: string, value: any) {
    const path = await resolveResource(".settings.dat");
    const store = new Store(path);
    await store.set(key, value)
    await store.save()
}


export async function get_value(key: string) {
    const path = await resolveResource(".settings.dat");
    const store = new Store(path);
    return await store.get(key)
}
