## 技术栈

- Vue3
- Rust
- Tauri

## 功能点
- 扫码登录
- 注销登录
- 搜索视频
- 下载视频
- 下载视频并转换成mp3格式
- 播放视频
- 循环播放列表视频


## 部分截图
<div style="display: flex; justify-content: space-around;">
    <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/not_login.jpg" alt="Not Login" style="width: 49%;">
    <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/login.jpg" alt="Login" style="width: 49%;">
</div>
<div style="display: flex; justify-content: space-around;">
 <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/search.jpg" alt="Search" style="width: 49%;">
  <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/playlist.jpg" alt="Playlist" style="width: 49%;">
</div>
<div style="display: flex; justify-content: space-around;">
    <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/pre_load.jpg" alt="Pre Load" style="width: 49%;">
    <img src="https://github.com/huxiaofan1223/bilimusic/raw/master/screenshorts/loading.jpg" alt="Loading" style="width: 49%;">
</div>

### 文档来自
- https://github.com/SocialSisterYi/bilibili-API-collect/
### 原理
- Rust请求B站接口返回给前端
- 图片和视频通过Rust反向代理获取
