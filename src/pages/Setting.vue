<script setup>
import { ref, onActivated, computed } from 'vue';
import { useStore } from '../store';
import { storeToRefs } from 'pinia';
import { invoke } from "@tauri-apps/api/tauri";
import QRCode from 'qrcode';
import { useMessage } from 'naive-ui'
const message = useMessage();
const store = useStore();
const { cookieVal, isLogin, userInfo } = storeToRefs(store);

const cookie_val = ref('');
const loginDialogVisible = ref(false);
const qrcode_key = ref('');
const qrCodeDataUrl = ref('');
let intervalId = null;
const limitSeconds = ref(180);
const userPngsrc = computed(() => {
    const url = btoa(userInfo.value.face || '');
    return `http://${store.fileProxyBaseURL}/?url=${url}`;
});
const generateQRCode = async (url) => {
    try {
        const dataUrl = await QRCode.toDataURL(url);
        qrCodeDataUrl.value = dataUrl;
    } catch (error) {
        console.error('Error generating QR code:', error);
    }
};

const saveSetting = () => {
    const val = cookie_val.value;
    store.setCookie(val);
}

const handleScanCode = async () => {
    const result = await invoke("get_login_png_url");
    limitSeconds.value = 180;
    console.log({ result });
    const jsonResult = JSON.parse(result);
    const url = jsonResult.data.url;
    qrcode_key.value = jsonResult.data.qrcode_key;
    await generateQRCode(url);
    loginDialogVisible.value = true;
    loopQueryLogin();
}
const loopQueryLogin = async () => {
    console.log('loop');
    clearInterval(intervalId);
    intervalId = setInterval(async () => {
        const resultString = await invoke("get_loop_login_query", { qrcodekey: qrcode_key.value });
        console.log(resultString);
        const parseResult = JSON.parse(resultString);
        const res = JSON.parse(parseResult.result);
        const code = res.data.code;
        const msg = res.data.message;
        const cookie = parseResult.cookie;
        console.log({ msg });
        if (code === 0) {
            clearInterval(intervalId);
            message.success('登录成功!');
            cookie_val.value = cookie;
            store.setCookie(cookie);
            loginDialogVisible.value = false;
            setTimeout(() => {
                store.getUserInfo();
            }, 200)
        } else {
            if (msg !== '未扫码') {
                message.error(msg);
            }
        }
        if (limitSeconds.value > 0) {
            limitSeconds.value -= 1;
        } else {
            handleScanCode();
            message.success('二维码已过期,已更新二维码');
        }
        console.log({ parseResult });
    }, 1000);
}
const onAfterLeave = () => {
    clearInterval(intervalId);
}
onActivated(() => {
    cookie_val.value = cookieVal.value;
})
const handlePositiveClick = () => {
    handleLoginout();
    message.info('退出登录成功');
}
const handleNegativeClick = () => {
    // message.info('取消');
}
const handleLoginout = () => {
    store.handleLoginout();
    cookie_val.value = '';
}
</script>

<template>
    <div style="display: flex;flex-direction: column;height:calc(100vh - 50px);padding:10px;">
        <div style="margin-bottom:12px;display: flex;">
            <label for="" style="margin-right:10px;width:80px;display: flex;align-items: center;">登录状态:</label>
            <div style="flex:1;align-items: center;display: flex;">
                <span v-if="isLogin" style="align-items: center;display: flex;">
                    <img :src="userPngsrc" alt="" style="width:30px;height:30px;border-radius: 50%;margin-right:10px;">
                    {{ userInfo.uname }}
                </span>
                <span style="margin:0 10px;">{{ isLogin ? '已登录' : '未登录' }}</span>
                <n-button v-if="!isLogin" :disabled="isLogin" size="small" type="primary"
                    @click="handleScanCode">扫码登录</n-button>

                <n-popconfirm positive-text="确定" negative-text="取消" @positive-click="handlePositiveClick"
                    @negative-click="handleNegativeClick" v-if="isLogin">
                    <template #trigger>
                        <n-button size="small" type="primary">退出登录</n-button>
                    </template>
                    是否退出登录?
                </n-popconfirm>
            </div>
        </div>
        <div style="display: flex;flex:1;align-items: flex-start;">
            <label for="" style="margin-right:10px;width:80px;">Cookie:</label>
            <n-input v-model:value="cookie_val" type="textarea" :disabled="true" placeholder="Cookie" style="flex:1;" />
        </div>
        <!-- <div style="text-align: center;">
            <n-button size="small" type="primary" @click="saveSetting">保存</n-button>
        </div> -->
    </div>
    <n-modal v-model:show="loginDialogVisible" title="扫码登录" preset="dialog" :mask-closable="false"
        :on-after-leave="onAfterLeave">
        <div style="text-align: center;">
            <img :src="qrCodeDataUrl" alt="">
        </div>
        <div style="text-align: center;">
            <div style="margin-bottom:5px;">请打开Bilibili客户端扫码</div>
            还有 <span style="color:red;">{{ limitSeconds }}</span> 秒过期
        </div>
    </n-modal>
</template>

<style></style>