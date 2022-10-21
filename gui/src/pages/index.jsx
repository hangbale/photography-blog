import BlogList from "components/blog-list"
import { getBlogList } from 'service';
import { useEffect, useState } from "react";
export default function Home() {
    let [list, setList] = useState([]);
    function fetchList () {
        getBlogList().then(res => {
            setList(res || [])
        })
    }
    useEffect(fetchList, []);
    return (
        <div className="page-home">
            <BlogList list={list}/>
        </div>
    )
}