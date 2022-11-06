import { Button, Text } from "@nextui-org/react";
import { open } from '@tauri-apps/api/dialog';
import { setConfig, getConfig, PicLibPath } from "service/config";
import { getImgList } from "service";
import { useEffect, useState } from "react";
import ImgList from "components/img-list";
import { Tracker } from 'service/utils';
import Swal from 'sweetalert2'
let TrackerIns = new Tracker();

export default function Pics () {
    let [libPath, setPath] = useState('')
    let [list, setList] = useState(null)
    
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
          TrackerIns.setIndex(res || '');
          getDirImgs(res || '')
      })
    }
    function getDirImgs (path) {
        getImgList(path).then(res => {
            if (res.imgs && res.imgs.length === 0) {
                Swal.fire('该目录为空')
            } else {
                console.log(res)
                setList(res)
                TrackerIns.add(res)
            }
        })
    }
    function onOpenDir (item) {
        getDirImgs(item.path)
    }
    function goBack () {
        let t = TrackerIns.getPre()

        if (t) {
            setList(t)
        }
    }
    useEffect(fetchConfig, [])
    // useEffect(fetchImg, [])
    return (
        <div className="page-pics">
            <div className="flex">
                <Button size="sm" className="mr-16" color="secondary" onPress={setLib}>设置图片库</Button>
                <Text  className="page-title">{libPath}</Text>
                <Button
                    className="flex-right"
                    size="sm"
                    onPress={goBack}
                    color="primary"
                    icon={<i className="bi bi-arrow-left-circle-fill"></i>}
                >返回</Button>
            </div>
            <div className="img-list-block mt-16">
                <ImgList onOpenDir={onOpenDir} list={list} galleryId="indexImgList"/>
            </div>
        </div>
    )
}