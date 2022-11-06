import BlogList from "components/blog-list"
import { getBlogList } from 'service';
import { useEffect, useState } from "react";
import { Button } from "@nextui-org/react";
import AddBlog from 'components/add-blog';

export default function Home() {
    let [list, setList] = useState([]);
    let [showAdd, setShowAdd] = useState(false);
    function fetchList () {
        getBlogList().then(res => {
            setList(res || [])
        })
    }
    function onAdd () {
        setShowAdd(true)
    }
    useEffect(fetchList, []);
    return (
        <div className="page-home">
            <div>
                <Button size="sm" className="mr-16" color="secondary" onPress={onAdd}>新增</Button>
            </div>
            <BlogList list={list}/>
            <AddBlog onClose={() => setShowAdd(false)} show={showAdd}/>
        </div>
    )
}