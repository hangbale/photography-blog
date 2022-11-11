import { Table, Row, Col, Tooltip, Button, Popover } from '@nextui-org/react';
import { removeBlog } from "service";
import Swal from 'sweetalert2'

export default function BlogList (props) {
    if (!props.list) {
        return null
    }
    let rows = props.list
    const columns = [
        {
            key: "name",
            label: "名称",
        },
        {
            key: "ip",
            label: "IP",
        },
        {
            key: "domain",
            label: "域名",
        },
        {
            key: "lastUpdate",
            label: "最近发布",
        },
        {
            key: "actions",
            label: "操作",
        },
    ];
    function remove (item) {
        removeBlog(item.key).then(res => {
            Swal.fire(res.data)
            if (res.code === 0) {
                props.onRefresh()
            }
        })
    }
    function renderCell (list, columnKey) {
        const cellValue = list[columnKey];
        switch (columnKey) {
            case "name":
                return (
                    <>{list.name}</>
                );
            case "ip":
                return (
                    <>{list.ip}</>
                );
            case "actions":
                return (
                    <Row justify="center" align="center">
                        <Col css={{ d: "flex" }}>
                            <Button
                                light
                                color="primary"
                                auto
                                icon={<i className="bi bi-eye-fill"></i>}></Button>
                        </Col>
                        <Col css={{ d: "flex" }}>
                            <Button
                                light
                                color="primary"
                                auto
                                icon={<i className="bi bi-pencil-fill"></i>} />
                        </Col>
                        <Col css={{ d: "flex" }}>
                            <Popover placement="top">
                                <Popover.Trigger>
                                    <Button
                                        auto
                                        color="error"
                                        icon={<i className="bi bi-trash-fill"></i>} />
                                </Popover.Trigger>
                                <Popover.Content>
                                    <Button
                                        auto
                                        color="error"
                                        onClick={() => remove(list)}
                                        icon={<i className="bi bi-trash-fill"></i>}>确认删除</Button>
                                </Popover.Content>
                            </Popover>
                            
                        </Col>
                    </Row>
                );
            default:
                return cellValue;
        }
    };
    return (
        <Table
            aria-label="Example table with dynamic content"
            css={{
                height: "auto",
                minWidth: "100%",
            }}
            className="main-bg mt-16"
        >
            <Table.Header columns={columns}>
                {(column) => (
                    <Table.Column width={column.key === 'actions' ? '200px': ''} key={column.key}>{column.label}</Table.Column>
                )}
            </Table.Header>
            <Table.Body items={rows}>
                {(item) => (
                    <Table.Row key={item.key}>
                        {(columnKey) => <Table.Cell>{renderCell(item, columnKey)}</Table.Cell>}
                    </Table.Row>
                )}
            </Table.Body>
        </Table>
    );
}
