<script setup lang="ts">
import type { GenericElectionCreate } from '@/backend';
import type { PropType } from 'vue';
import CreateCandidates from '@/components/CreateCandidates.vue';

defineProps({
    modelValue: {
        type: Object as PropType<GenericElectionCreate>,
        required: true,
    }
});

</script>

<template>
    <div>
        <CreateCandidates v-model="modelValue.options" :min_candidates="2" />
    </div>
</template>

<style scoped></style>
