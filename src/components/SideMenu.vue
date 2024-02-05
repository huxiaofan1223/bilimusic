<script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
const goPath = (val)=>{
    router.push({ path: val||'/' });
}
</script>

<template>
    <div class="menu">
        <div class="menu-item" :class="{ 'active': $route.path === '/' }" @click="goPath('/')">搜索</div>
        <div class="menu-item" :class="{ 'active': $route.path === '/playlist' }" @click="goPath('/playlist')">列表</div>
        <div class="menu-item" :class="{ 'active': $route.path === '/setting' }" @click="goPath('/setting')">设置</div>
    </div>
</template>

<style scoped>
.menu {
    width: 100px;
    background: white;
    text-align: center;
    height: 100%;
    margin-left:-15px;
    .menu-item {
        height: 40px;
        line-height: 40px;
        cursor: pointer;
        transition: all .3s;
        font-size: 14px;

        &:hover {
            background: #f8f8f8;
            color: #36ad6a;
        }
    }

    .active {
        background: #f8f8f8;
        color: #36ad6a;
    }
}
</style>