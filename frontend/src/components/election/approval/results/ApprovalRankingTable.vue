<script setup lang="ts">
import type { ApprovalResult } from '@/backend';
import type { PropType } from 'vue';

defineProps({
    options: {
        type: Object as PropType<string[]>,
        required: true,
    },
    result: {
        type: Object as PropType<ApprovalResult>,
        required: true,
    },
});

</script>

<template>
    <div>
        <table>
            <tr>
                <th>Name</th>
                <th>Approval</th>
                <th>Rank</th>
            </tr>
            <tr v-for="(tally, i) in result.approve_tally">
                <td>{{ options[tally.option_index] }}</td>
                <td>{{ (tally.approval_count / result.vote_count * 100).toFixed(2) }}% ({{ tally.approval_count }})</td>
                <td>{{ i + 1 }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped></style>
