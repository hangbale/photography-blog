import { Input, Row, Col, Container, Modal, Button } from "@nextui-org/react";
import { useState, useEffect } from "react";
import { addBlog } from 'service';
import Swal from 'sweetalert2'

export default function AddBlog (props) {
    let [nameState, setNameState] = useState('default')
    let [values, setValues] = useState({})
    function onChange (key, v) {
        setValues({
            ...values,
            [key]: v
        })
    }
    function onSubmit () {
        if (!values.name) {
            setNameState('error')
            return
        }
        console.log(values)
        addBlog(values).then(res => {
            console.log(res)
            Swal.fire('操作成功')
        })
    }
    useEffect(() => {
        console.log(props)
    }, [props])
    return (
        <Modal
            closeButton
            aria-labelledby="modal-title"
            open={props.show}
            onClose={props.onClose}
        >
            <Container>
                <h3>新增博客</h3>
                <Row className="pt-36">
                    <Col span={12}>
                        <Input value={values.name}
                            onBlur={(e) => onChange('name', e.target.value)}
                            status={nameState} required 
                            fullWidth clearable bordered labelPlaceholder="名称" />
                    </Col>
                </Row>
                <Row className="pt-36">
                    <Col span={12}>
                        <Input value={values.ip} onBlur={(e) => onChange('ip', e.target.value)} fullWidth clearable bordered labelPlaceholder="IP" />
                    </Col>
                </Row>
                <Row className="pt-36">
                    <Col span={12}>
                        <Input value={values.domain} onBlur={(e) => onChange('domain', e.target.value)} fullWidth clearable bordered labelPlaceholder="域名" />
                    </Col>
                </Row>
            </Container>
            <Modal.Footer>
                <Button auto flat color="error" onClick={props.onClose}>
                    取消
                </Button>
                <Button auto onClick={onSubmit}>
                    确定
                </Button>
            </Modal.Footer>
        </Modal>
    )
}
