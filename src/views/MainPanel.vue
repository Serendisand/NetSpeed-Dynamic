<template>
    <div class="panel-container">
        <header class="panel-header">
            <div class="brand">
                <img src="../assets/logo.png" class="logo-icon">
                <div>
                    <h1>NetSpeed Dynamic</h1>
                    <p class="subtitle">NSD 桌面动态组件 v1.0.0</p>
                </div>
            </div>

            <div class="header-controls">
                <span class="status-badge" :class="{ 'is-active': isWidgetVisible }">
                    {{ isWidgetVisible ? '已开启' : '已关闭' }}
                </span>
                <label class="switch header-switch">
                    <input type="checkbox" :checked="isWidgetVisible" @change="toggleWidget">
                    <span class="slider"></span>
                </label>
            </div>
        </header>

        <hr class="divider" />

        <div class="main-content">
            <div class="card status-card">
                <h3>当前实时状态</h3>
                <div class="speed-monitor">
                    <div class="speed-item">
                        <span class="arrow up">↑</span>
                        <div class="speed-info">
                            <span class="label">上传速度</span>
                            <span class="value">{{ uploadSpeed }}</span>
                        </div>
                    </div>
                    <div class="speed-item">
                        <span class="arrow down">↓</span>
                        <div class="speed-info">
                            <span class="label">下载速度</span>
                            <span class="value">{{ downloadSpeed }}</span>
                        </div>
                    </div>
                </div>
                <div ref="chartRef" class="mini-chart"></div>
            </div>

            <div class="card settings-card">
                <h3>常规设置</h3>

                <div class="setting-item is-disabled">
                    <div class="item-meta">
                        <span class="item-title">开机自动启动 <span class="tag-dev">未实现</span></span>
                        <span class="item-desc">跟随系统启动 NSD</span>
                    </div>
                    <label class="switch">
                        <input type="checkbox" v-model="autoStart" disabled>
                        <span class="slider"></span>
                    </label>
                </div>

                <div class="setting-item slider-item">
                    <div class="item-meta">
                        <span class="item-title">悬浮窗不透明度</span>
                        <span class="item-desc">调节灵动岛的外观透明度 ({{ opacity }}%)</span>
                    </div>
                    <input type="range" min="0" max="100" v-model="opacity" class="range-input" />
                </div>
            </div>
        </div>

        <footer class="panel-footer">
            <span>&copy; 2026 Ryen. All rights reserved.</span>
            <span class="action-link">检查更新</span>
        </footer>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';

// 【核心改动】直接从全局 window 中获取 echarts 对象，规避 TS 编译报错
const echarts = (window as any).echarts;

const isWidgetVisible = ref(false);
const autoStart = ref(false);
const opacity = ref(Number(localStorage.getItem('nsd_island_opacity') || '100'));

// 实时速度文本
const uploadSpeed = ref('0 B/s');
const downloadSpeed = ref('0 B/s');

// 照抄灵动岛的数据计算变量
let lastRx = 0;
let lastTx = 0;
let speedTimer: number;

// ECharts 实例及队列
const chartRef = ref<HTMLElement | null>(null);
let chartInstance: any = null;
const chartDataQueue: number[] = Array(15).fill(0); // 预留15个初始0点，让图表顺滑滚动

// 格式化速度显示
const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

// 初始化 ECharts 折线图
const initChart = () => {
    if (!chartRef.value || !echarts) return;
    chartInstance = echarts.init(chartRef.value);

    const option = {
        grid: {
            top: 5,
            bottom: 5,
            left: 0,
            right: 0,
        },
        xAxis: {
            type: 'category',
            boundaryGap: false,
            show: false,
        },
        yAxis: {
            type: 'value',
            show: false,
            min: 0,
        },
        series: [
            {
                data: chartDataQueue,
                type: 'line',
                smooth: true,          // 平滑折线
                symbol: 'none',        // 隐藏圆点
                lineStyle: {
                    color: '#3b82f6',  // 科技蓝主色
                    width: 2,
                },
                // 渐变面积覆盖
                areaStyle: {
                    color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
                        { offset: 0, color: 'rgba(59, 130, 246, 0.4)' },
                        { offset: 1, color: 'rgba(59, 130, 246, 0.0)' }
                    ]),
                },
            },
        ],
    };

    chartInstance.setOption(option);
};

// 照抄灵动岛的流量获取与计算原理
const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;

            // 1. 更新文字显示
            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);

            // 2. 计算下行流量，换算为纯数字 MB/s
            const speedMB = rxDiff / (1024 * 1024);

            // 3. 更新图表队列数据
            chartDataQueue.push(Number(speedMB.toFixed(2)));
            if (chartDataQueue.length > 15) {
                chartDataQueue.shift();
            }

            // 4. 推动图表刷新
            chartInstance?.setOption({
                series: [{ data: chartDataQueue }]
            });
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('控制台流量获取失败:', error);
    }
};

// 监听滑块变化
watch(opacity, async (newVal) => {
    localStorage.setItem('nsd_island_opacity', newVal.toString());
    await emit('control-island-opacity', { opacity: newVal });
});

onMounted(async () => {
    // 初始化图表
    initChart();

    // 开启流量计算定时器（1秒同步一次）
    fetchSpeedStats();
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;

    // 窗口缩放自适应尺寸
    window.addEventListener('resize', () => chartInstance?.resize());

    // 监听来自灵动岛的自发性隐藏通知
    await listen<{ visible: boolean }>('island-status-sync', (event) => {
        isWidgetVisible.value = event.payload.visible;
    });

    for (let i = 0; i < 6; i++) {
        try {
            const visible = await invoke<boolean>('is_widget_visible');
            if (visible) {
                isWidgetVisible.value = true;
                return;
            }
        } catch { /* 忽略 */ }
        await new Promise(r => setTimeout(r, 200));
    }
    isWidgetVisible.value = false;
});

onUnmounted(() => {
    clearInterval(speedTimer);
    chartInstance?.dispose();
});

const toggleWidget = async () => {
    const nextState = !isWidgetVisible.value;
    await emit('control-island-visibility', { show: nextState });
    isWidgetVisible.value = nextState;
};
</script>

<style scoped>
/* 全局样式基础重置 */
:global(body) {
    background-color: #f8fafc;
    color: #1e293b;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', Roboto, sans-serif;
    margin: 0;
    padding: 0;
    user-select: none;
    -webkit-font-smoothing: antialiased;
}

.panel-container {
    padding: 28px 32px;
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    min-height: calc(100vh - 56px);
}

.panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
}

.brand {
    display: flex;
    align-items: center;
    gap: 16px;
}

.logo-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
}

.brand h1 {
    font-size: 20px;
    margin: 0;
    font-weight: 700;
    letter-spacing: 0.2px;
    color: #0f172a;
}

.subtitle {
    font-size: 13px;
    color: #64748b;
    margin: 4px 0 0 0;
}

.header-controls {
    display: flex;
    align-items: center;
    gap: 16px;
    background: #ffffff;
    padding: 8px 12px 8px 16px;
    border-radius: 24px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
    border: 1px solid #e2e8f0;
}

.status-badge {
    font-size: 13px;
    font-weight: 600;
    color: #94a3b8;
    transition: all 0.3s;
}

.status-badge.is-active {
    color: #2b2b2b;
}

.divider {
    border: none;
    border-top: 1px solid #e2e8f0;
    margin-bottom: 24px;
}

.main-content {
    display: grid;
    grid-template-columns: 1fr 1.3fr;
    gap: 24px;
    flex-grow: 1;
}

.card {
    background: #ffffff;
    border: 1px solid #e2e8f0;
    border-radius: 20px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    box-shadow: 0 4px 20px -2px rgba(0, 0, 0, 0.03);
    transition: transform 0.2s, box-shadow 0.2s;
}

.card:hover {
    box-shadow: 0 8px 24px -4px rgba(0, 0, 0, 0.06);
}

.card h3 {
    font-size: 15px;
    color: #334155;
    margin: 0 0 20px 0;
    font-weight: 600;
}

.speed-monitor {
    display: flex;
    flex-direction: column;
    gap: 20px;
    margin-bottom: 24px;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 16px;
}

.arrow {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 800;
    font-size: 16px;
}

.arrow.up {
    background: #eff6ff;
    color: #3b82f6;
}

.arrow.down {
    background: #ecfdf5;
    color: #10b981;
}

.speed-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.speed-info .label {
    font-size: 12px;
    color: #64748b;
    font-weight: 500;
}

.speed-info .value {
    font-size: 18px;
    font-weight: 700;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    color: #0f172a;
    letter-spacing: -0.5px;
}

/* 波动图表 Canvas 容器 */
.mini-chart {
    width: 100%;
    height: 80px;
    margin-top: auto;
    padding-top: 16px;
    border-top: 1px solid #f1f5f9;
}

.setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 0;
    border-bottom: 1px solid #f1f5f9;
}

.setting-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
}

.slider-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
}

.item-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.item-title {
    font-size: 14px;
    font-weight: 600;
    color: #1e293b;
    display: flex;
    align-items: center;
    gap: 8px;
}

.tag-dev {
    font-size: 10px;
    background: #f1f5f9;
    color: #64748b;
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: normal;
}

.item-desc {
    font-size: 13px;
    color: #64748b;
}

.is-disabled {
    opacity: 0.5;
    pointer-events: none;
}

.switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 28px;
}

.switch input {
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
    background-color: #cbd5e1;
    transition: 0.4s cubic-bezier(0.4, 0.0, 0.2, 1);
    border-radius: 28px;
}

.slider:before {
    position: absolute;
    content: "";
    height: 22px;
    width: 22px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    transition: 0.4s cubic-bezier(0.4, 0.0, 0.2, 1);
    border-radius: 50%;
}

input:checked+.slider {
    background-color: #2b2b2b;
}

input:checked+.slider:before {
    transform: translateX(20px);
}

input:disabled+.slider {
    background-color: #e2e8f0;
    cursor: not-allowed;
}

.range-input {
    width: 100%;
    -webkit-appearance: none;
    appearance: none;
    background: #e2e8f0;
    height: 6px;
    border-radius: 3px;
    outline: none;
}

.range-input::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #ffffff;
    border: 2px solid #2b2b2b;
    cursor: pointer;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
    transition: transform 0.1s;
}

.range-input::-webkit-slider-thumb:hover {
    transform: scale(1.1);
}

.panel-footer {
    margin-top: 32px;
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: #94a3b8;
    font-weight: 500;
}

.action-link {
    color: #2b2b2b;
    cursor: pointer;
    transition: color 0.2s;
}

.action-link:hover {
    color: #2b2b2b;
    text-decoration: underline;
}
</style>