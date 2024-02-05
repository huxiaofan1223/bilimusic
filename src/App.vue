<script setup>
import { ref, onMounted } from 'vue';
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import SideMenu from "./components/SideMenu.vue";
import Player from "./components/Player.vue";
import { useStore } from './store';
import { useMessage } from 'naive-ui'

const store = useStore();
const dragTarget = ref(null);
let isDragging = false;
let startX = 0;
let startY = 0;
let startLeft = 0;
let startTop = 0;

const startDrag = (e) => {
  isDragging = true;
  startX = e.pageX - dragTarget.value.offsetLeft;
  startY = e.pageY - dragTarget.value.offsetTop;
  startLeft = dragTarget.value.offsetLeft;
  startTop = dragTarget.value.offsetTop;
};

const doDrag = (e) => {
  // Check if the mouse is dragging or just moving
  if (isDragging) {
    // Prevent other interactions during dragging

    const newLeft = e.pageX - startX;
    const newTop = e.pageY - startY;

    // Ensure the element stays within the horizontal boundaries
    const maxX = window.innerWidth - dragTarget.value.clientWidth;
    const clampedLeft = Math.max(0, Math.min(newLeft, maxX));

    // Ensure the element stays within the vertical boundaries
    const maxY = window.innerHeight - dragTarget.value.clientHeight;
    const clampedTop = Math.max(0, Math.min(newTop, maxY));

    dragTarget.value.style.left = clampedLeft + 'px';
    dragTarget.value.style.top = clampedTop + 'px';
  }
};

const stopDrag = (e) => {
  isDragging = false;
};

onMounted(() => {
  store.initCookie();
  setTimeout(()=>{
    store.getUserInfo();
    store.initFileProxyBaseURL();
  },100)
})

</script>

<template>
  <n-message-provider>
    <div style="padding:15px;display: flex;margin-top:0;">
      <SideMenu />
      <div style="flex:1;">
        <!-- <Search /> -->
        <router-view v-slot="{ Component }">
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </router-view>
      </div>

      <div style="position:fixed;left:10px;bottom:10px;width:360px;" ref="dragTarget" @mousedown="startDrag"
        @mousemove="doDrag" @mouseup="stopDrag">
        <Player />
      </div>
    </div>
  </n-message-provider>
</template>

<style>
/* .dplayer-video-wrap .dplayer-video{
  max-height:300px;
} */
* {
  box-sizing: border-box;
}
</style>
