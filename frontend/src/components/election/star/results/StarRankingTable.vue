<script setup lang="ts">
import type { StarResult } from '@/backend';
import type { PropType } from 'vue';
import { rank_to_emoji } from '@/utils';

defineProps({
    options: {
        type: Object as PropType<string[]>,
        required: true,
    },
    result: {
        type: Object as PropType<StarResult>,
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
            <tr v-for="(tally, i) in result.points_tally">
                <td>{{ options[tally.option_index] }}</td>
                <td>{{ tally.points_count }}</td>
                <td :class="i <= 1 ? `runoff` : ``">{{ rank_to_emoji(i + 1) }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.runoff {
    background-color: #74ef97;
}
</style>
