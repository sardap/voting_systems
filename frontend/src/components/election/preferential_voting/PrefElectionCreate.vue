<script setup lang="ts">
import type { PrefCreateElection } from '@/backend';
import type { PropType } from 'vue';
import CreateCandidates from '@/components/CreateCandidates.vue';

defineProps({
    modelValue: {
        type: Object as PropType<PrefCreateElection>,
        required: true,
    }
});

</script>

<template>
    <div>
        <CreateCandidates v-model="modelValue.options" />
    </div>
</template>

<style scoped>
hr {
    margin-top: 3px;
    margin-bottom: 8px;
}
</style>
