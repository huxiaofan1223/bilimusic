<script setup>
import { reactive, ref, createVNode, h } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useStore } from '../store';
import { NButton } from 'naive-ui';
import { useMessage } from 'naive-ui'
import { useRouter } from 'vue-router';
import { createFFmpeg, fetchFile } from '@ffmpeg/ffmpeg'

const router = useRouter();
const goPath = (val) => {
  router.push({ path: val || '/' });
}
const message = useMessage();

const store = useStore();
const keyword = ref("");
const searchResult = ref({});
const tableData = ref([]);
const loading = ref(false);
const downloadVideoName = ref('下载视频');
const downloadVideoURL = ref('');
const progressVal = ref('');
const progressWidth = ref('0%');
const downloadVideoFileName = ref('');
const pagination = reactive({
  page: 1,
  pageCount: 1,
  pageSize: 20,
  itemCount: 0,
  prefix({ itemCount }) {
    return `总共${itemCount}条`
  }
})
const isDownloading = ref(false);
const isTransform = ref(false);
const isTransformTxt = ref('');
const downloadVisible = ref(false);
const columns = [
  {
    title: '封面',
    key: 'pic',
    width: 60,
    render(row) {
      const url = row.pic.replace(/^(?:http:)?\/\//, 'https://');
      const newUrl = `http://${store.fileProxyBaseURL}?url=${btoa(url)}`;
      return createVNode('img', {
        src: newUrl,
        alt: '',
        style: {
          width: '50px',
          height: '50px',
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
    title: '作者',
    key: 'author',
    ellipsis: true,
    width: 100
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
      return h(NButton, {
        size: 'small',
        type: 'primary',
        onClick: (event) => {
          handleDownloadVideoShow(row);
          event.stopPropagation(); // 阻止事件冒泡
        }
      }, { default: () => '下载' });
    }
  },
]

async function getSearchResult() {
  if (!store.isLogin) {
    message.error('请先登录bilibili账号');
    goPath('/setting');
    return;
  }
  loading.value = true;
  const result = await invoke("get_search_result", { keyword: keyword.value, page: pagination.page + '' });
  const jsonResult = JSON.parse(result);
  // console.log(result);
  loading.value = false;
  console.log({ jsonResult });
  if (jsonResult.code === 0) {
    searchResult.value = jsonResult;
    tableData.value = jsonResult.data.result;
    pagination.itemCount = jsonResult.data.numResults;
  } else {
    message.error(jsonResult.message);
  }
}
const rowProps = (row) => {
  return {
    style: 'cursor: pointer;',
    onClick: async () => {
      const result = await invoke("get_video_url", { id: row.id + '' });
      const jsonResult = JSON.parse(result);
      console.log({ jsonResult });
      if (jsonResult.code === 0) {
        const url = btoa(jsonResult.data.durl[0].url);
        const localURL = `http://${store.fileProxyBaseURL}/?url=${url}`;
        const localURLTimeoutStamp = new Date().getTime() + 120 * 60 * 1000;  //120分钟视频url过期
        store.unshiftList({ ...row, localURL, localURLTimeoutStamp });
      }
    }
  }
}
function handlePageChange(currentPage) {
  if (!loading.value) {
    pagination.page = currentPage;
    getSearchResult();
  }
}
function byte2Txt(size) {
  const unitArr = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB', 'BB'];
  let count = 0;
  while (size / 1024 > 1) {
    size = size / 1024
    count++;
  }
  return size.toFixed(2) + unitArr[count];
}

const handleDownloadVideoShow = async (row) => {
  console.log(row);
  downloadVisible.value = true;
  const result = await invoke("get_video_url", { id: row.id + '' });
  const jsonResult = JSON.parse(result);
  console.log({ jsonResult });
  if (jsonResult.code === 0) {
    const url = btoa(jsonResult.data.durl[0].url);
    const localURL = `http://${store.fileProxyBaseURL}/?url=${url}`;
    const size = jsonResult.data.durl[0].size;
    const sizeName = byte2Txt(size);
    downloadVideoFileName.value = row.title.replace(/<.*?>/g, '');
    downloadVideoName.value = jsonResult.data.accept_description[0] + ` (${sizeName})`;
    downloadVideoURL.value = localURL;
  }
}
const handleDownloadVideo = async () => {
  isDownloading.value = true;
  const videoUrl = downloadVideoURL.value;
  fetch(videoUrl)
    .then(response => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      const reader = response.body.getReader();
      const contentLength = +response.headers.get('Content-Length');
      const blobChunks = [];
      let receivedBytes = 0;
      function read() {
        reader.read().then(({ done, value }) => {
          if (done) {
            const blob = new Blob(blobChunks, { type: 'video/mp4' });
            const downloadLink = document.createElement('a');
            downloadLink.href = URL.createObjectURL(blob);
            downloadLink.download = downloadVideoFileName.value + '.mp4'; // 设置下载文件的名称
            document.body.appendChild(downloadLink);
            downloadLink.click();
            document.body.removeChild(downloadLink);
            isDownloading.value = false;
            progressVal.value = '';
            progressWidth.value = '0%';
          } else {
            receivedBytes += value.byteLength;
            const progress = (receivedBytes / contentLength) * 100;
            progressWidth.value = progress + '%';
            const progressName = byte2Txt(receivedBytes) + '/' + byte2Txt(contentLength);
            progressVal.value = `${progressName} ${progress.toFixed()}%`;
            blobChunks.push(value);
            read();
          }
        });
      }
      read();
    })
    .catch(error => {
      console.error('Error fetching video:', error)
      isDownloading.value = false;
      progressVal.value = '';
      progressWidth.value = '0%';
    });
}
const onClose = () => {
  if (isDownloading.value) {
    message.error('请耐心等待下载完再关闭窗口');
    return false;
  } else {
    downloadVisible.value = false;
    progressVal.value = '';
    progressWidth.value = '0%';
  }
}

const handleDownloadAudio = async () => {
  isDownloading.value = true;
  let videoUrl = downloadVideoURL.value;
  // videoUrl = 'http://127.0.0.1/test.mp4';
  const fileName = downloadVideoFileName.value + '.mp3';
  const response = await fetch(videoUrl);
  if (response.body) {
    const contentLength = +response.headers.get('content-length');
    let receivedLength = 0;

    const reader = response.body.getReader();
    const stream = new ReadableStream({
      start(controller) {
        function push() {
          reader.read().then(({ done, value }) => {
            if (done) {
              controller.close();
              // isDownloading.value = false;
              // progressVal.value = '';
              // progressWidth.value = '0%';
              return;
            }
            receivedLength += value.byteLength;
            const progress = (receivedLength / contentLength) * 100;
            progressWidth.value = progress + '%';
            const progressName = byte2Txt(receivedLength) + '/' + byte2Txt(contentLength);
            progressVal.value = `${progressName} ${progress.toFixed()}%`;
            controller.enqueue(value);
            push();
          });
        }
        push();
      }
    });

    const readableStream = new Response(stream);
    const videoData = new Uint8Array(await readableStream.arrayBuffer());
    const worker = new Worker("ffmpeg-worker-mp4.js");
    isTransform.value = true;
    worker.onmessage = function (e) {
      const msg = e.data;
      switch (msg.type) {
        case "ready":
          console.log('ready')
          worker.postMessage({ type: "run", MEMFS: [{ name: "input.mp4", data: videoData }], arguments: ['-i', 'input.mp4', '-vn', '-acodec:a', 'libmp3lame', '-q:a', '0', 'output.mp3'] });
          break;
        case "run":
          console.log("run", msg.data);
          break;
        case "stdout":
          console.log('stdout', msg.data);
          break;
        case "stderr":
          isTransformTxt.value = msg.data;
          console.log('stderr', msg.data);
          break;
        case "error":
          console.log("error", msg.data);
          break;
        case "exit":
          console.log("exit", msg.data);
          break;
        case "abort":
          console.log("abort", msg.data);
          break;
        case "done":
          isDownloading.value = false;
          progressVal.value = '';
          progressWidth.value = '0%';
          isTransform.value = false;
          console.log('done', msg.data);
          const audioData = msg.data.MEMFS[0].data;
          const blob = new Blob([audioData], { type: 'audio/mp3' });
          const url = URL.createObjectURL(blob);
          const a = document.createElement('a');
          a.href = url;
          console.log([fileName]);
          a.download = fileName; // 设置文件名
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
          URL.revokeObjectURL(url);
          break;
      }
    };
  }
}
</script>

<template>
  <div style="margin-bottom:10px;">
    <n-input-group style="width:300px;">
      <n-input placeholder="请输入" v-model:value="keyword" v-on:keyup.enter="getSearchResult" />
      <n-button type="primary" @click="getSearchResult">
        搜索
      </n-button>
    </n-input-group>
  </div>
  <n-data-table remote ref="table" :columns="columns" :data="tableData" :loading="loading" size="small"
    :pagination="pagination" :row-key="() => { return 'id' }" @update:page="handlePageChange" :row-props="rowProps"
    max-height="calc(100vh - 160px)" />
  <n-modal v-model:show="downloadVisible" title="下载" preset="dialog" :mask-closable="false" :onClose="onClose">
    <div>
      <div>
        <n-button @click="handleDownloadVideo" :loading="isDownloading">{{ downloadVideoName }}</n-button>
        <n-button @click="handleDownloadAudio" :loading="isDownloading" type="primary"
          style="margin-left:30px;">只下载音频</n-button>
      </div>
      <div v-if="isDownloading" style="margin-top:10px;">
        <div>进度:&nbsp;&nbsp;{{ progressVal }}</div>
        <div :style="{ width: progressWidth, height: '5px', backgroundColor: '#4CAF50', borderRadius: '2px' }"></div>
      </div>

      <div v-if="isTransform" style="margin-top:10px;">音频转换中...</div>
      <div v-if="isTransform" style="margin-top:10px;">{{ isTransformTxt }}</div>
    </div>
  </n-modal>
</template>
