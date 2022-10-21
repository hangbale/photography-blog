# Personal Photography Blog Generator
摄影类博客生成器

> The theme was originally made by HTML5 UP [Multiverse | HTML5 UP](https://html5up.net/multiverse)



DEMO: http://i.idinr.com

 [**中文说明**](https://github.com/hangbale/photography-blog/wiki/%E4%BD%BF%E7%94%A8%E8%AF%B4%E6%98%8E)

## screenshots

![](https://github.com/hangbale/photography-blog/blob/master/screenshots/pc.png?raw=true)   
![](https://github.com/hangbale/photography-blog/blob/master/screenshots/mobile.png?raw=true)

## Change Log
### 2022-6-14
- Show Image Exif Info
### 2022-6-30
- explain how to develop a theme
### 2022-7-23
- bug fix on windows


## Features

- Fully Responsive
- HTML5 + CSS3
- FontAwesome Icons
- Multi-level Albums Support
- Basic Breadcrumbs
- Automated Image Fit
- Easy Config With JSON
- Faster
- auto parse Exif data



## Usage

1. Download the executable file from release page, put it to your project directory.

2. Create a file `config.json` in project directory.

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

   **note:**

   - The root node will be used to generate home page.
   - A image collection must have  **children** and **title** field.
   - If a item is collection then  generate a page for it.
   - A single image must have **url** field.

3. Excute generator   

   you should add executable permission first.
   ```bash
   cd project_dir
   ./blog-cli
   
   ```
   for windows: excute blog-cli.exe

4. A `public` directory will be created, use it as your server root

## Image and Exif
There are two ways to store image: cdn or local.
Only four Exif fields will be displayed:  

- focal
- iso
- aperture
- shutter

### CDN (recommended)
If you want show Exif of a image in cdn，you should the field：`extra` of its parent **collection node**.  

A cdn url must start with http of https to distinguish from local image uri.

```
   "extra": {
    "image_exif_query_suffix": "?exif",
    "image_style_suffix": "$blog"
  }
```
image_exif_query_suffix: to fetch url exif data from cdn api.
image_style_suffix: if the cdn url has a image style suffix the replace it to get original url.

**note**: I wrote this code because i use `qiniu(七牛云)`，I did not handle other cdn service because they are too many.

### local
- create a directory named by `image`
- put all your iamges in
- specify the url field with prefix `/image`，because this mean the root of website

**A local image will be extracted Exif info automatically.**

### Specify Exif manually
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
    },
```
 ## Theme
Because all the pages use a same html template currently, so a theme can only have one html file named `index.html`. 
Other assets should be placed in a directory named `assets`
 ### development
 The template engine is `Tera`
 
 **directory structure**
 `assets` and `index.html` are mandatory.

 - theme root
   - assets  
   - index.html
 
 **available variables in template**
 - all the fields in `config.json`
 - item.exif // the EXIF info of image
    - parsed true/false if local parse exif info successed
    - focal 
    - iso 
    - shutter
    - aperture
- item.category album/none is a image collection
- breadcrumbs // array of breadcrumb
  - title // path title
  - path // path url
  - current true/false //is current path

## Development

It is a Rust Application.

### Dependencies

- Rust Environment
- Tera
- Serde
- Serde_json
- Urlencoding

### Default Values

Defined in ` src/config.rs`

### Template
The default template is   `template/index.html` and be used by all the pages.
Template Engine is Tera https://tera.netlify.app/docs/.

