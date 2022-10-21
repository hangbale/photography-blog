import { BaseDirectory } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir, exists } from '@tauri-apps/api/fs';
const mainConfig = 'config.json';
export const PicLibPath = 'default_pic_lib'

function wirteFile (path, content) {
    return writeTextFile(path, content, { dir: BaseDirectory.App });
}
function readFile (path) {
    return readTextFile(path, { dir: BaseDirectory.App });
}
function tryFile () {
    return new Promise(function (resolve, reject) {
        exists('', { dir: BaseDirectory.App }).then(res => {
            console.log(res)
            if (res) {
                resolve(true)
            } else {
                createDir('', { dir: BaseDirectory.App, recursive: true }).then(res => {
                    console.log('created', res)
                    resolve(true)
                }).catch(e => {
                    console.log('create fail', e)
                    reject(e)
                })
            }
        }).catch(e => {
            console.log('exist err:', e)
        })
    })
}
export function setConfig (key, v) {
    let p = new Promise(function (resolve, reject) {
        tryFile().then(res => {
            console.log('try', res)
            readFile(mainConfig).then(res => {
                console.log(res)
                let c = res ? JSON.parse(res) : {}
                resolve(c)
            }).catch(e => {
                console.log('read file error:', e)
                wirteFile(mainConfig, '{}').then(res => {
                    resolve({})
                }).catch(e => {
                    console.log('write config error:', e)
                })
            })
        }).catch(e => {
            console.log('try file error:', e)
            reject(e)
        })
    })
    p.then(res => {
        res[key] = v
        return wirteFile(mainConfig, JSON.stringify(res))
    })
    return p
}
export function getConfig (key) {
    return new Promise(function (resolve, reject) {
        readFile(mainConfig).then(res => {
            if (!res) {
                resolve(null)
            }
            let t = JSON.parse(res)
            if (!t[key]) {
                resolve(null)
            }
            resolve(t[key])
        }).catch(e => {
            resolve(null)
        })
    })
}