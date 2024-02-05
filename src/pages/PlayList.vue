<script setup>
import { reactive, ref, createVNode, h } from "vue";
import { useStore } from '../store';
import { storeToRefs } from 'pinia';
import { NButton } from 'naive-ui';
import { useMessage } from 'naive-ui'
import { useRouter } from 'vue-router';
import bus from 'vue3-eventbus';

const router = useRouter();
const message = useMessage();
const store = useStore();
const { playList } = storeToRefs(store);
const tableData = ref(playList);

const rowProps = (row, rowIndex) => {
    return {
        style: 'cursor: pointer;',
        onClick: async () => {
            console.log({ row, rowIndex });
            console.log(rowIndex);
            store.setIdx(rowIndex);
        }
    }
}
const rowClassName = (row, index) => {
    return index === store.idx ? 'rowActive' : ''
}
const columns = [
    {
        title: '封面',
        key: 'pic',
        width: 50,
        render(row) {
            const url = row.pic.replace(/^(?:http:)?\/\//, 'https://');
            const newUrl = `http://${store.fileProxyBaseURL}?url=${btoa(url)}`;
            return createVNode('img', {
                src: newUrl,
                alt: '',
                style: {
                    width: '30px',
                    height: '30px',
                    borderRadius: '5px',
                    objectFit: 'cover'
                }
            });
        }
    },
    {
        title: '标题',
        key: 'title',
        // width: 400,
        ellipsis: true,
        render(row) {
            return createVNode('div', {
                innerHTML: row.title
            });
        }
    },
    {
        title: '时长',
        key: 'duration',
        ellipsis: true,
        width: 100,
        render(row) {
            return row.duration.replace(/(?<=^|:)(?=\d(?:$|:))/g, '0');
        }
    },
    {
        title: '操作',
        key: 'operation',
        width: 100,
        render(row, rowIndex) {
            console.log(rowIndex);
            return h(NButton, {
                size: 'small',
                type: 'error',
                onClick: (event) => {
                    store.removeIdx(rowIndex);
                    event.stopPropagation(); // 阻止事件冒泡
                }
            }, { default: () => '删除' });
        }
    },
]
// 在这里监听事件
bus.on('getURLErrorToLogin',()=>{
    message.error('获取链接失败,请先登录试试！');
    router.push('/setting');
});

</script>

<template>
    <n-data-table remote ref="table" size="small" :columns="columns" :data="tableData" :row-key="() => { return 'id' }"
        :row-props="rowProps" :row-class-name="rowClassName" max-height="calc(100vh - 80px)" />
</template>

<style scoped>
:deep(.rowActive td) {
    color: #36ad6a !important;
}
</style>