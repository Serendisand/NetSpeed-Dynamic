<template>
    <div class="blank-dynamic-page">
        <h3 class="page-title">实时活动设置已预留</h3>
        <p class="page-desc">未来可以在这里添加实时活动相关配置...</p>
    </div>
</template>

<script setup lang="ts">
// 实时活动相关的逻辑/API引用可以在此处扩展
</script>

<style scoped>
.blank-dynamic-page {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 300px;
    color: var(--item-desc-color);
}

.page-title {
    margin: 0 0 8px 0;
    color: var(--item-title-color);
}

.page-desc {
    margin: 0;
    font-size: 13px;
}
</style>