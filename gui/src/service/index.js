import { invoke } from '@tauri-apps/api'
export function getBlogList () {
    return invoke('greet');
}
