<script setup lang="ts">
import type { PropType } from 'vue';

const props = defineProps({
    candidates: {
        required: true,
        type: Array as PropType<string[]>,
    }
});
</script>

<template>
    <table>
        <tr>
            <th>Name</th>
        </tr>
        <tr v-for="candidate in candidates">
            <td>{{ candidate }}</td>
        </tr>
    </table>
</template>

<style scoped></style>
