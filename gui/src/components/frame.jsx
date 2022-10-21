import { Text } from "@nextui-org/react";
import { useLocation, useNavigate } from 'react-router-dom';
import { Outlet } from "react-router-dom";

function getActive (path) {
    let c = ['/', '/pic'];
    let key = ['blog', 'pic'];
    return key[c.indexOf(path)];
}

export default function PageFrame (props) {
    let location = useLocation();
    let active = getActive(location.pathname);
    const navigate = useNavigate();
    function goto(p) {
        navigate(p);
    }
    return (
        <div className="main-frame">
            <aside className="left-nav">
                <Text
                    h1
                    size={40}
                    css={{
                        textGradient: "45deg, $blue600 -20%, $pink600 50%",
                    }}
                    weight="bold"
                >
                    PicTank
                </Text>
                <ul className="nav-list">
                    <li onClick={() => goto('/')}>
                        <Text h4 className={active === 'blog' ? 'active' : ''}>
                            <i className={`bi bi-cloud-arrow-up${active === 'blog' ? '-fill' : ''}`}></i>
                            <span className="nav-text">
                                博客
                            </span>
                        </Text>
                    </li>
                    <li onClick={() => goto('/pic')}>
                        <Text h4 className={active === 'pic' ? 'active' : ''}>
                            <i className={`bi bi-collection${active === 'pic' ? '-fill' : ''}`}></i>
                            <span className="nav-text">
                                相簿
                            </span>
                        </Text>
                    </li>
                </ul>
            </aside>
            <aside className="right-content">
                <main className="main-wrapper">
                    <Outlet />
                </main>
            </aside>
        </div>
    )
}