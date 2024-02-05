// store/index.js
import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import bus from 'vue3-eventbus';
export const useStore = defineStore({
    persist: true,
    id: 'playList',
    state: () => ({
        idx: 0,
        playList: [],
        cookieVal: "",
        isLogin: false,
        userInfo: {

        },
        fileProxyBaseURL: ''
    }),
    actions: {
        unshiftList(row) {
            const playlisIds = this.playList.map(item => item.id);
            const idx = playlisIds.indexOf(row.id);
            console.log(playlisIds)
            console.log(row.id);
            if (!playlisIds.includes(row.id)) {
                this.playList.unshift(row);
                this.setIdx(0)
            } else {
                this.playList[idx] = row;
                this.setIdx(idx)
            }
        },
        async setIdx(idx) {
            console.log('setidx', idx);
            const row = this.playList[idx];
            console.log(row.localURLTimeoutStamp, new Date().getTime());
            if (row.localURLTimeoutStamp < new Date().getTime()) {
                console.log('视频url过期');
                const result = await invoke("get_video_url", { id: row.id + '' });
                const jsonResult = JSON.parse(result);
                console.log({ jsonResult });
                if (jsonResult.code === 0) {
                    try {
                        const url = btoa(jsonResult.data.durl[0].url);
                        const localURL = `http://${this.fileProxyBaseURL}/?url=${url}`;
                        const localURLTimeoutStamp = new Date().getTime() + 120 * 60 * 1000;  //120分钟视频url过期
                        this.playList[idx].localURL = localURL;
                        this.playList[idx].localURLTimeoutStamp = localURLTimeoutStamp;
                    } catch (err) {
                        if (!this.isLogin) {
                            bus.emit('getURLErrorToLogin');
                            return;
                        }
                    }
                }
            }
            const url = this.playList[idx] && this.playList[idx].localURL;
            if (idx === this.idx) {
                bus.emit('setVideoUrl', { url, toggle: true });
            } else {
                bus.emit('setVideoUrl', { url, toggle: false });
            }
            this.idx = idx;
        },
        removeIdx(idx) {
            if (idx <= this.idx) {
                if (this.idx !== 0) {
                    this.idx -= 1;
                }
            }
            this.playList.splice(idx, 1);
            setTimeout(() => {
                const url = this.playList[this.idx] && this.playList[this.idx].localURL;
                bus.emit('setVideoUrl', { url, toggle: false });
            }, 100);
        },
        async setCookie(val) {
            this.cookieVal = val;
            const result = await invoke("set_cookie_val", { cookieval: val });
            console.log(result);
        },
        async initCookie() {
            const val = this.cookieVal;
            const result = await invoke("set_cookie_val", { cookieval: val });
            console.log(result);
        },
        async getUserInfo() {
            const result = await invoke("get_user_info");
            console.log(result);
            const res = JSON.parse(result);
            if (res.code === 0) {
                this.isLogin = true;
                this.userInfo = res.data;
            } else {
                this.isLogin = false;
            }
        },
        async initFileProxyBaseURL() {
            console.log('initFileProxyBaseURL');
            const result = await invoke("get_file_proxy_baseurl");
            this.fileProxyBaseURL = result;
        },
        handleLoginout() {
            this.isLogin = false;
            this.userInfo = {};
            this.setCookie('');
        }
    },
    getters: {

    }
});