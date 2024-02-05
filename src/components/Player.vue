<script setup>
import { reactive, ref, createVNode, watch, onMounted } from "vue";
import DPlayer from 'dplayer';
import { useStore } from '../store';
import bus from 'vue3-eventbus';
const store = useStore();

// onMounted(() => {
//     watch([() => store.playList, () => store.idx], ([newPlayList, newIdx], [oldPlayList, oldIdx]) => {
//         console.log('playList changed', newPlayList, oldPlayList);
//         console.log('idx changed', newIdx, oldIdx);
//         const url = newPlayList[newIdx] && newPlayList[newIdx].localURL;
//         setVideoUrl(url);
//     }, { immediate: false, deep: true });
// })

let dp;
const setVideoUrl = ({ url, toggle }) => {
    if (!dp) {
        if (url) {
            dp = new DPlayer({
                container: document.getElementById('dplayer'),
                autoplay: true,
                video: {
                    url
                }
            });
            dp.on('ended', function () {
                console.log('player ended');
                let newIdx = store.idx + 1;
                if (newIdx === store.playList.length) {
                    newIdx = 0;
                }
                store.setIdx(newIdx);
            });
        } else {
            dp.destroy();
        }
    } else {
        if (url) {
            if (dp.video.src !== url) {
                console.log('change url', dp.video.src, url);
                dp.switchVideo({
                    url: url,
                });
                dp.play();
            } else {
                if (toggle) {
                    dp.toggle();
                }
            }
        }
    }
}


// 在这里监听事件
bus.on('setVideoUrl', setVideoUrl);

</script>


<template>
    <div id="dplayer"></div>
</template>