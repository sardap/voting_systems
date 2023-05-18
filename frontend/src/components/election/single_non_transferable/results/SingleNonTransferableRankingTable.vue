<script setup lang="ts">
import type { SntvResult } from '@/backend';
import type { PropType } from 'vue';

defineProps({
    result: {
        type: Object as PropType<SntvResult>,
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
                <th>Elected</th>
                <th>Rank</th>
            </tr>
            <tr v-for="(tally, i) in result.vote_tally">
                <td>{{ result.options[tally.option_index] }}</td>
                <td>{{ (tally.vote_count / result.votes.length * 100).toFixed(2) }}% ({{ tally.vote_count }})</td>
                <td> {{ result.winners.includes(tally.option_index) ? `🥇` : `` }}</td>
                <td>{{ i + 1 }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped></style>
