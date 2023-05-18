<script setup lang="ts">
import type { ThreeTwoOneResult } from '@/backend';
import type { PropType } from 'vue';
import { rank_to_emoji } from '@/utils';

defineProps({
    result: {
        type: Object as PropType<ThreeTwoOneResult>,
        required: true,
    },
});

</script>

<template>
    <div>
        <table>
            <tr>
                <th>Name</th>
                <th>Semifinalist</th>
                <th>Finalist</th>
                <th>Good😊</th>
                <th>Ok😐</th>
                <th>Bad😞</th>
                <th>Score</th>
            </tr>
            <tr v-for="(tally, i) in result.points_tally">
                <td>{{ result.options[tally.option_index] }}</td>
                <td>{{ result.semifinalists.includes(tally.option_index) ? "✅" : "❌" }}</td>
                <td>{{ result.finalists.includes(tally.option_index) ? "✅" : "❌" }}</td>
                <td>{{ tally.good_count }}</td>
                <td>{{ tally.ok_count }}</td>
                <td>{{ tally.bad_count }}</td>
                <td>{{ tally.score }}</td>
            </tr>
        </table>
    </div>
</template>

<style scoped>
.runoff {
    background-color: #74ef97;
}
</style>
