<template>
    <div class="dynamic-set-dashboard">

        <div class="grid-section">

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <path
                            d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83" />
                    </svg>
                    <span>动态与物理反馈</span>
                </div>
                <div class="spring-selector">
                    <button class="spring-btn" :class="{ active: springStyle === 'stiff' }"
                        @click="springStyle = 'stiff'">
                        <svg viewBox="0 0 24 24" class="spring-icon">
                            <path d="M4 12 L8 8 L12 16 L16 8 L20 12" stroke="currentColor" stroke-width="2" fill="none"
                                stroke-linejoin="round" />
                        </svg>
                        <span>克制 (快/稳)</span>
                    </button>
                    <button class="spring-btn" :class="{ active: springStyle === 'bouncy' }"
                        @click="springStyle = 'bouncy'">
                        <svg viewBox="0 0 24 24" class="spring-icon">
                            <path d="M3 12 C 7 2, 10 22, 14 12 C 16 7, 18 16, 21 12" stroke="currentColor"
                                stroke-width="2" fill="none" stroke-linecap="round" />
                        </svg>
                        <span>Q弹 (活泼)</span>
                    </button>
                </div>
            </div>

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                        <circle cx="8.5" cy="8.5" r="1.5" />
                        <polyline points="21 15 16 10 5 21" />
                    </svg>
                    <span>外观与边缘</span>
                </div>
                <div class="form-group-list">
                    <div class="form-item">
                        <span class="label">边缘形态</span>
                        <div class="shape-toggle">
                            <button :class="{ active: borderRadius === 100 }" @click="borderRadius = 100"
                                title="经典胶囊"></button>
                            <button :class="{ active: borderRadius === 16 }" @click="borderRadius = 16" title="圆角矩形"
                                style="border-radius: 6px;"></button>
                        </div>
                    </div>
                    <div class="form-item mt-auto">
                        <span class="label">炫彩流光边框</span>
                        <label class="mock-switch">
                            <input type="checkbox" v-model="isGlowBorderEnabled">
                            <span class="slider"></span>
                        </label>
                    </div>
                </div>
            </div>

            <div class="neo-card">
                <div class="card-header">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round" class="title-icon">
                        <line x1="12" y1="5" x2="12" y2="19" />
                        <line x1="5" y1="12" x2="19" y2="12" />
                    </svg>
                    <span>桌面坐标偏移</span>
                </div>
                <div class="stepper-group">
                    <div class="stepper-row">
                        <span class="axis">Y轴</span>
                        <div class="stepper-control">
                            <button @click="offsetY -= 5">-</button>
                            <input type="text" :value="offsetY + 'px'" readonly />
                            <button @click="offsetY += 5">+</button>
                        </div>
                    </div>
                    <div class="stepper-row">
                        <span class="axis">X轴</span>
                        <div class="stepper-control">
                            <button @click="offsetX -= 5">-</button>
                            <input type="text" :value="offsetX + 'px'" readonly />
                            <button @click="offsetX += 5">+</button>
                        </div>
                    </div>
                </div>
            </div>

        </div>

        <div class="list-section">
            <div class="card-header" style="margin-bottom: 20px;">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                    stroke-linejoin="round" class="title-icon">
                    <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
                </svg>
                <span style="font-size: 16px; letter-spacing: 0.5px;">尺寸边界空间</span>
            </div>

            <div class="slider-list-container">
                <div class="slider-row">
                    <div class="row-info">
                        <span class="row-title">常态驻留宽度</span>
                        <span class="row-desc">控制待机时的长度 (默认 150px)</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="100" max="300" v-model="baseWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ baseWidth }}<span class="unit">px</span></div>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <span class="row-title">全局高度基准</span>
                        <span class="row-desc">影响所有状态下的厚度 (默认 34px)</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="24" max="60" v-model="baseHeight"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ baseHeight }}<span class="unit">px</span></div>
                    </div>
                </div>

                <div class="slider-row">
                    <div class="row-info">
                        <span class="row-title">消息律动展开宽度</span>
                        <span class="row-desc">音乐播放或长消息的最大边界</span>
                    </div>
                    <div class="row-action">
                        <input type="range" min="260" max="600" v-model="expandedWidth"
                            class="track-slider highlight-slider" />
                        <div class="value-box">{{ expandedWidth }}<span class="unit">px</span></div>
                    </div>
                </div>
            </div>
        </div>

    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

// 尺寸状态
const baseWidth = ref(150);
const baseHeight = ref(34);
const expandedWidth = ref(320);

// 形态与外观
const borderRadius = ref(100); // 100为胶囊, 16为圆角矩形
// 直接接管本地缓存
const isGlowBorderEnabled = ref(localStorage.getItem('nsd_glow_border') === 'true');

// 物理动效
const springStyle = ref<'stiff' | 'bouncy'>('bouncy');

// 位置偏移 (桌面端新加需求) 
const offsetX = ref(0);
const offsetY = ref(10);

// 统一监听更新逻辑入口
watch([baseWidth, baseHeight, expandedWidth, borderRadius, isGlowBorderEnabled, springStyle, offsetX, offsetY], () => {
    // 写入本地缓存以便 WidgetIsland 实时感知
    localStorage.setItem('nsd_glow_border', String(isGlowBorderEnabled.value));

    // TODO: 预留给 IPC 发送或 Pinia 的状态同步
    // console.log('UI 配置变更，触发底层渲染...');
}, { deep: true });
</script>

<style scoped>
/* 全局容器 (绝对禁止滚动，铺满高度) */
.dynamic-set-dashboard {
    display: flex;
    flex-direction: column;
    gap: 20px;
    height: 100%;
    max-height: calc(100vh - 120px);
    overflow: hidden;
    box-sizing: border-box;
    user-select: none;
}

/* 宫格区域 */
.grid-section {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    flex-shrink: 0;
}

/* 现代科技感卡片 */
.neo-card {
    background: var(--card-bg);
    border: 1px solid var(--card-border);
    border-radius: 16px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    transition: transform 0.2s, border-color 0.2s;
}

.card-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: var(--item-title-color);
    /* 修复：跟随主面板标题色 */
    margin-bottom: 16px;
}

.title-icon {
    width: 16px;
    height: 16px;
    color: var(--item-desc-color);
    /* 修复：跟随主面板描述文字颜色 */
}

/* --- 卡片1：物理选择器 --- */
.spring-selector {
    display: flex;
    gap: 10px;
    height: 100%;
}

.spring-btn {
    flex: 1;
    background: transparent;
    /* 修复：跟随系统二级按钮背景 */
    border: 1px solid var(--control-border);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 8px;
    color: var(--btn-sec-color);
    /* 修复：跟随二级按钮文字颜色 */
    cursor: pointer;
    transition: all 0.2s;
}

.spring-icon {
    width: 24px;
    height: 24px;
}

.spring-btn:hover {
    background: var(--btn-pri-bg);
    color: var(--btn-sec-hover-color);
}

.spring-btn.active {
    background: var(--btn-pri-bg);
    /* 修复：激活时使用主色调（深色模式下为白/浅灰，浅色模式下为深黑） */
    border-color: var(--btn-pri-border);
    color: var(--btn-pri-color);
    box-shadow: 0 2px 8px var(--card-shadow-hover);
}

/* --- 卡片2：形态与开关 --- */
.form-group-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: 100%;
}

.form-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.mt-auto {
    margin-top: auto;
}

.label {
    font-size: 13px;
    color: var(--item-title-color);
    /* 修复：调高对比度，防止浅色模式看不清 */
}

.shape-toggle {
    display: flex;
    gap: 6px;
}

.shape-toggle button {
    width: 32px;
    height: 20px;
    background: var(--btn-pri-bg);
    border: 2px solid transparent;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s;
}

.shape-toggle button.active {
    border-color: var(--item-title-color);
    background: var(--btn-pri-bg);
}

/* 完美复刻原UI的Switch开关 */
.mock-switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 24px;
}

.mock-switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--slider-bg);
    /* 修复 */
    transition: .3s;
    border-radius: 24px;
    border: 1px solid var(--control-border);
}

.slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: #ffffff;
    /* 保持滑块纯白 */
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
    transition: .3s;
    border-radius: 50%;
}

input:checked+.slider {
    background-color: var(--slider-checked-bg);
    /* 修复 */
    border-color: var(--slider-checked-bg);
}

input:checked+.slider:before {
    transform: translateX(20px);
}

/* --- 卡片3：步进器 (坐标微调) --- */
.stepper-group {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    height: 100%;
    gap: 10px;
}

.stepper-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: var(--btn-sec-bg);
    /* 修复 */
    padding: 6px 8px;
    border-radius: 8px;
    border: 1px solid var(--control-border);
}

.axis {
    font-size: 12px;
    font-weight: bold;
    color: var(--item-desc-color);
    /* 修复 */
    width: 30px;
    text-align: center;
}

.stepper-control {
    display: flex;
    align-items: center;
    background: var(--bg-body);
    /* 修复 */
    border-radius: 6px;
    border: 1px solid var(--control-border);
    overflow: hidden;
}

.stepper-control button {
    width: 28px;
    height: 24px;
    background: transparent;
    border: none;
    color: var(--item-title-color);
    /* 修复 */
    cursor: pointer;
    font-weight: bold;
}

.stepper-control button:hover {
    background: var(--btn-pri-bg);
    color: var(--btn-pri-color);
}

.stepper-control input {
    width: 44px;
    text-align: center;
    background: transparent;
    border: none;
    color: var(--item-title-color);
    /* 修复 */
    font-size: 13px;
    font-family: monospace;
    pointer-events: none;
}

/* ================= 列表区域 (尺寸精调) ================= */
.list-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--control-bg, rgba(255, 255, 255, 0.03));
    border: 1px solid var(--card-border, rgba(255, 255, 255, 0.08));
    border-radius: 16px;
    padding: 20px;
}

.section-title {
    font-size: 15px;
    font-weight: 600;
    color: var(--item-title-color);
    margin-bottom: 20px;
    letter-spacing: 0.5px;
}

.slider-list-container {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    /* 均分剩余高度，避免挤在一起 */
    flex: 1;
}

.slider-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
}

.row-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.row-title {
    font-size: 15px;
    font-weight: 500;
    color: #fff;
}

.row-desc {
    font-size: 12px;
    color: #888;
}

.row-action {
    display: flex;
    align-items: center;
    gap: 20px;
}

/* 现代化大滑块 */
.track-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 180px;
    height: 8px;
    background: var(--slider-bg, rgba(255, 255, 255, 0.1));
    border-radius: 4px;
    outline: none;
}

.track-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
    cursor: pointer;
    transition: transform 0.1s;
}

.track-slider::-webkit-slider-thumb:hover {
    transform: scale(1.2);
}

.highlight-slider::-webkit-slider-thumb {
    border: 3px solid #666;
    /* 强提示滑块 */
}

/* 数据展示框 */
.value-box {
    width: 64px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-body, rgba(0, 0, 0, 0.3));
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    color: #fff;
    font-family: ui-monospace, monospace;
}

.value-box .unit {
    font-size: 10px;
    color: #666;
    margin-left: 2px;
}
</style>