<template>
    <div class="panel-container">
        <h1>NetSpeed 控制面板</h1>
        <p>桌面悬浮组件控制中心</p>

        <div class="card">
            <div class="setting-item">
                <span>显示灵动岛悬浮窗</span>
                <button @click="toggleWidget">
                    {{ isWidgetVisible ? '关闭' : '显示' }} 灵动岛
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event'; // 新增事件监听
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

const isWidgetVisible = ref(false);
let unlisten: (() => void) | null = null; // 类型安全初始化

onMounted(async () => {
    // ✅ 修正2：先监听事件再查询状态
    unlisten = await listen('widget_shown', () => {
        isWidgetVisible.value = true;
    });

    // ✅ 修正3：删除不必要的延迟（事件驱动不需要）
    const widget = await WebviewWindow.getByLabel('widget');
    if (widget) {
        isWidgetVisible.value = await widget.isVisible();
    }
});

// ✅ 修正4：安全取消监听
onUnmounted(() => {
    if (unlisten) {
        unlisten();
        unlisten = null; // 避免内存泄漏
    }
});

const toggleWidget = async () => {
    const widget = await WebviewWindow.getByLabel('widget');
    if (!widget) return;

    if (isWidgetVisible.value) {
        await widget.hide();
        isWidgetVisible.value = false; // 直接设 false
    } else {
        await widget.show();
        await widget.setAlwaysOnTop(true);
        isWidgetVisible.value = true; // 直接设 true
    }
    // ✅ 修正5：删除这行！（前面已直接赋值，再反转会出错）
    // isWidgetVisible.value = !isWidgetVisible.value; ← 必须删除！
};
</script>

<style scoped>
:global(body) {
    background-color: #1e1e1e;
    color: white;
    font-family: Arial, sans-serif;
}

.panel-container {
    padding: 30px;
}

.card {
    background: #2a2a2a;
    border-radius: 12px;
    padding: 20px;
    margin-top: 20px;
}

.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

button {
    padding: 8px 16px;
    border-radius: 8px;
    border: none;
    background: #444;
    color: white;
    cursor: pointer;
    transition: 0.2s;
}

.btn-active {
    background: #0A84FF;
}
</style>