<script setup lang="ts">
import type { CumulativeResult } from '@/backend';
import { rank_to_emoji } from '@/utils';
import type { PropType } from 'vue';

defineProps({
    result: {
        type: Object as PropType<CumulativeResult>,
        required: true,
    },
});

</script>

<template>
    <div>
        <table>
            <tr>
                <th>Name</th>
                <th>Score</th>
                <th>Rank</th>
            </tr>
            <tr v-for="(tally, i) in result.votes_tally">
                <td>{{ result.options[tally.option_index] }}</td>
                <td>{{ tally.vote_count }}</td>
                <td>{{ rank_to_emoji(i + 1) }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped></style>
