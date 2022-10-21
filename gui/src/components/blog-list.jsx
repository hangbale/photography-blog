import { Table, Row, Col, Tooltip, Button } from '@nextui-org/react';

export default function BlogList () {
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
            key: "status",
            label: "最近发布",
        },
        {
            key: "actions",
            label: "操作",
        },
    ];
    const rows = [
        {
            key: "1",
            name: "Tony Reichert",
            ip: "12.12.12.12",
            status: "Active",
        },
        {
            key: "2",
            name: "Zoey Lang",
            ip: "12.12.12.12",
            status: "Paused",
        },
        {
            key: "3",
            name: "Jane Fisher",
            ip: "12.12.12.12",
            status: "Active",
        },
        {
            key: "4",
            name: "William Howard",
            ip: "12.12.12.12",
            status: "Vacation",
        },
    ];
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
                            <Tooltip content="Details">
                                <Button
                                    light
                                    color="primary"
                                    auto
                                    icon={<i className="bi bi-eye-fill"></i>}
                                />
                            </Tooltip>
                        </Col>
                        <Col css={{ d: "flex" }}>
                            <Tooltip content="Edit user">
                                <Button
                                    light
                                    color="primary"
                                    auto
                                    icon={<i className="bi bi-pencil-fill"></i>}
                                />
                            </Tooltip>
                        </Col>
                        <Col css={{ d: "flex" }}>
                            <Tooltip
                                content="Delete user"
                                color="error"
                                onClick={() => console.log("Delete user")}
                            >
                                <Button
                                    auto
                                    color="error"
                                    icon={<i className="bi bi-trash-fill"></i>}
                                />
                                
                            </Tooltip>
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
            className="main-bg"
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
