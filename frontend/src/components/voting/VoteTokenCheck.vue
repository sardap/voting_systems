<script setup lang="ts">
import { onMounted, ref, type PropType } from 'vue';
import type { GenericElection } from '@/backend';

const props = defineProps({
    election: {
        type: Object as PropType<GenericElection>,
        required: true,
    },
    vote_token: {
        type: String,
        required: false,
    }
});


const loading = ref<boolean>(true);
const error = ref<String | null>(null)

onMounted(async () => {
    if (props.election.require_token && !props.vote_token) {
        error.value = "This election requires a token to vote";
        return;
    }

    loading.value = false;
});
</script>

<template>
    <div>
        <div v-if="error" class="error-text">
            <p>Error {{ error }}</p>
        </div>
    </div>
</template>

<style scoped></style>
