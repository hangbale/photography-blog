import LightGallery from 'lightgallery/react';
import 'lightgallery/css/lightgallery.css';
import 'lightgallery/css/lg-zoom.css';
import 'lightgallery/css/lg-thumbnail.css';
import { useEffect } from 'react'
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { Grid, Card, Row, Col, Text, Button } from "@nextui-org/react";


export default function ImgList (props) {
    let list = props.list || null;

    function gotoDir (item) {
        props.onOpenDir(item)
    }
    if (!props.list || !props.list.name || !props.galleryId) {
        return null
    }
    return (
        <div className="pswp-gallery" id={ props.galleryId}>

            <LightGallery selector=".pic-list-ele" elementClassNames="pic-list" speed={500}>

            {
                list.imgs.map((image, index) => {
                    let url = convertFileSrc(image.path);
                    let exif = (image.exif && image.exif.parsed && image.exif.focal) ? image.exif : null;
                    return (
                        <>
                            {
                                image.imgType === 'img' ? (
                                    <div className="pic-list-item pic-list-ele" md={2} sm={2} xs={2} key={index} data-src={url}>
                                    <Card data-src={url}  css={{ w: "100%", h: "14vw" }}>
                                        <Card.Body css={{ p: 0 }}>
                                            <a
                                                href={url}
                                                data-src={url}
                                                target="_blank"
                                                rel="noreferrer"
                                                style={{ overflow: "auto" }}
                                            >
                                                <Card.Image
                                                    src={url}
                                                    data-src={url}
                                                    width="100%"
                                                    height="100%"
                                                    objectFit="cover"
                                                />
                                            </a>
                                        </Card.Body>
                                        {
                                            exif ? (
                                                    <Card.Footer
                                                        isBlurred
                                                        css={{
                                                            position: "absolute",
                                                            bgBlur: "#bebebe66",
                                                            borderTop: "$borderWeights$light solid rgba(255, 255, 255, 0.2)",
                                                            bottom: 0,
                                                            zIndex: 1,
                                                        }}
                                                    >
                                                        <Row>
                                                            <div>
                                                                <div className="exif-info">
                                                                    <div className="exif-info-item aperture">{exif.aperture}</div>
                                                                    <div className="exif-info-item focal">{exif.focal}</div>
                                                                    <div className="exif-info-item iso">{exif.iso}</div>
                                                                    <div className="exif-info-item shutter">{exif.shutter}</div>
                                                                </div>
                                                                <div className="exif-info">
                                                                    <div className="exif-info-item">{exif.model}</div>
                                                                    <div className="exif-info-item">{exif.date}</div>
                                                                </div>
                                                            </div>
                                                            <div className="flex-right">
                                                                <Button icon={<i className="bi bi-pencil-fill"></i>} flat auto rounded color="secondary">
                                                                </Button>
                                                            </div>
                                                        </Row>
                                                    </Card.Footer>
                                            ) : null
                                        }
                                        
                                    </Card>
                                </div>
                                ) : <div className="pic-list-item"><div className="img-entry" onClick={() => gotoDir(image)}>
                                        <i className="text-primary img-entry-icon bi bi-folder-fill"></i>
                                        <Text color="primary" size={16}>
                                            {image.name}
                                        </Text>
                                </div></div>
                                
                            }
                            </>
                    )
                })
            }


            </LightGallery>
            </div >

    )
}