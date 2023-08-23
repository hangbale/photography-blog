# Personal Photography Blog Generator
摄影类博客生成器

> The theme was originally made by HTML5 UP [Multiverse | HTML5 UP](https://html5up.net/multiverse)



DEMO: http://i.idinr.com

 [**中文说明**](https://github.com/hangbale/photography-blog/wiki/%E4%BD%BF%E7%94%A8%E8%AF%B4%E6%98%8E)

## screenshots

![](https://github.com/hangbale/photography-blog/blob/master/screenshots/pc.png?raw=true)   
![](https://github.com/hangbale/photography-blog/blob/master/screenshots/mobile.png?raw=true)


## 特点

- Fully Responsive
- HTML5 + CSS3
- FontAwesome Icons
- Multi-level Albums Support
- Basic Breadcrumbs
- Automated Image Fit
- Easy Config With JSON
- Faster



## 使用说明
1. 新建一个文件夹并clone此项目作为项目文件夹

2. 在release页面下载对应系统的可执行文件至项目文件夹

3. 在项目根目录下建一个 `config.json` 文件作为配置文件

   config example:

   ```json
   {
     "title": "My Blog",
     "description": "My Blog description",
     "author": "author name",
     "children": [
       {
         "title": "a dog image",
         "description": "description of this image",
         "url": "a image url"
       },
       {
         "title": "a image collection,",
         "description": "description of this collection",
         "url": "a thumbnail of this collection",
         "children": [ // a collection must have children field
           {
             "url": "a image url"
           }
         ]
       }
     ]
   }
   ```

   **说明:**

   - json的根结点将生成网站的首页
   - json节点有两类：相册类和图片类
   - 相册类的节点会单独生成一个页面
   - 如果节点是一个相册类的节点，则必须包含 **children** 和 **title** 字段，title字段会用于生成url
   - 图片节点必须有url字段

4. 执行
   非Windows系统可能需要先chmod增加执行权限，Windows用户直接执行blog-cli.exe即可。

   ```bash
   cd project_dir
   ./blog-cli
   
   ```
   
5. 根目录下会生成一个 `public` 文件夹，将此文件夹作为网站根目录即可

## 图片处理及Exif数据
目前支持cdn图片及本地图片两种方式。
cdn图片的exif数据获取仅支持七牛云，其他cdn暂时没时间研究，因为实在太多。(七牛云图片上传可以使用我开发的一个空间管理插件：https://github.com/hangbale/qiniukit)

本地图片会自动读取exif数据。   
目前支持展示4种exif字段
- focal 焦距
- iso   iso
- aperture 光圈
- shutter 快门

### cdn图片
```
   "extra": {
    "image_exif_query_suffix": "?exif",
    "image_style_suffix": "$blog"
  }
```
自动获取cdn图片exif需要在相册节点中配置extra字段，该相册下所有图片将使用这个字段来获取exif，暂不支持单个图片声明extra。
这两个字段是为了兼容七牛云的exif数据api，image_exif_query_suffix 这个字段七牛云是固定的`?exif`，需要跟在图片地址后面，
image_style_suffix 是图片样式符，如果你使用了图片样式，获取exif时要从url中去除该字符串。

### 本地图片
本地图片的exif会自动获取

- 项目根目录建一个 `image`文件夹用来托管本地图片
- 本地图片的url必须以 `/image`开头，这代表从网站的根目录开始寻找，因为image目录下所有的图片会被移动到public/image下

### 手动声明exif

```
    {
        "title": "title",
        "url": "http://xx.com/ss.jpg",
        "exif": {
            "focal": "1",
            "iso": "100",
            "shutter": "1/1000",
            "aperture": "1/1"
        }
    }
```

## 主题

- 当前版本只支持单模板
- 模板名必须是 `index.html`
- 其他静态文件放必须到 `assets`目录下

### 主题开发
模板使用Tera引擎，按照其语法即可

目录结构
- 主题根目录
  - index.html
  - assets

模板可用变量
 - `config.json`中所有的字段
 - item.exif // 图片的exif信息
    - parsed true/false 本地图片是否解析exif成功
    - focal // 焦距
    - iso  // iso
    - shutter // 快门
    - aperture // 光圈
- item.category album/none 是否是图片集
- breadcrumbs // 面包屑导航数组
  - title // 标题
  - path // url
  - current true/false //是否当前path

使用主题时，将主题文件夹中的文件复制到template目录即可

## 开发

使用rust

### 依赖项

- Rust Environment
- Tera
- Serde
- Serde_json
- Urlencoding

### 默认配置

在根目录下的 ` src/config.rs`文件中

### 模板
目前仅支持单模板，默认模板位于 `template/index.html`   
所有的页面都会使用此主题
模板引擎使用 Tera https://tera.netlify.app/docs/



