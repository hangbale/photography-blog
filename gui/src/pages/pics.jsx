import { Button, Text } from "@nextui-org/react";
import { open } from '@tauri-apps/api/dialog';
import { setConfig, getConfig, PicLibPath } from "service/config";
import { useEffect, useState } from "react";
export default function Pics () {
    let [libPath, setPath] = useState('')
    function setLib () {
        open({
            directory: true,
            multiple: false,
        }).then(res => {
            if (res) {
                setConfig(PicLibPath, res).then(res => {
                    console.log(res)
                    if (res && res[PicLibPath]) {
                        setPath(res[PicLibPath])
                    }
                })
            }
        })
    }
    function fetchConfig () {
      getConfig(PicLibPath).then(res => {
          setPath(res || '')
      })
    }
    useEffect(fetchConfig, [])
    return (
        <div className="page-pics">
            <div className="flex">
                <Button size="sm" className="mr-16" color="secondary" onClick={setLib}>设置图片库</Button>
                <Text  className="page-title">{libPath}</Text>
            </div>
        </div>
    )
}