import { invoke } from '@tauri-apps/api'
export function getBlogList () {
    return invoke('list');
}
export function getImgList (item) {
    return invoke('get_dir_imgs', {
        path: item
    });
}
export function addBlog (item) {
    return invoke('add_blog', {
        item: item
    });
}
export function removeBlog (item) {
    return invoke('remove_blog', {
        itemKey: item
    });
}