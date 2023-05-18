<script setup lang="ts">
import type { CreateCumulativeElection } from '@/backend';
import { ref, type PropType } from 'vue';
import CreateCandidates from '@/components/CreateCandidates.vue';

const props = defineProps({
    modelValue: {
        type: Object as PropType<CreateCumulativeElection>,
        required: true,
    }
});

const emits = defineEmits<{
    (e: 'update:modelValue', value: CreateCumulativeElection): void
}>();


const max_votes = ref(props.modelValue.max_votes);

</script>

<template>
    <div>
        <div>
            <label for="max_votes">Max votes </label><br />
            <input type="number" id="max_votes" v-model="max_votes" @change="emits(`update:modelValue`, {
                ...props.modelValue,
                max_votes: max_votes,
            })" /><br />
        </div>
        <br />
        <CreateCandidates v-model="modelValue.options" />
    </div>
</template>

<style scoped>
hr {
    margin-top: 3px;
    margin-bottom: 8px;
}
</style>
